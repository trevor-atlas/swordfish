use crate::settings::AppConfig;
use crate::sqlite::SQLite;
use crate::utilities::cache_all_app_icons;
use chrono::prelude::DateTime;
use chrono::Utc;
use fuzzy_matcher::skim::SkimMatcherV2;
use ignore::WalkBuilder;
use rayon::prelude::*;
use rusqlite::{params, Result};
use std::cmp;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{ffi::OsString, time::Instant};
use swordfish_types::{DataSource, FileInfo, Query};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DSError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to locate data directory")]
    MissingDir(&'static str),
    #[error("Failed to load search directories from settings")]
    MissingSearchDir,
    #[error("Transaction Error")]
    TransactionError(#[from] rusqlite::Error),
}

pub fn score_files(query: &Query, directories: Vec<String>) -> Option<Vec<String>> {
    let search_string = query.search_string.clone();
    let start = Instant::now();
    let matcher = SkimMatcherV2::default();
    let mut scored_directories: Vec<(i64, String)> = directories
        .par_iter()
        .map(|directory| {
            let path = Path::new(directory);
            let is_app = path.extension() == Some(&OsString::from("app"));
            let mut score = matcher
                .fuzzy(directory, &search_string, true)
                .map(|res| res.0)
                .unwrap_or(0);

            if let Some(fname) = path.file_name() {
                let fname_score = matcher
                    .fuzzy(
                        fname.to_string_lossy().to_string().as_str(),
                        &search_string,
                        true,
                    )
                    .map(|res| res.0)
                    .unwrap_or(0);
                score += fname_score;
            }

            if is_app && score > 0 {
                // bumping app matches because they're more likely to be relevant
                score += 10;
            }

            (score, directory.to_owned())
        })
        .filter(|res| res.0 > 0)
        .collect();
    println!("finished search in {}ms", start.elapsed().as_millis());

    scored_directories.sort_by(|a, b| b.0.cmp(&a.0));
    Some(scored_directories.iter().map(|res| res.1.clone()).collect())
}

pub struct FileDataSource {
    sqlite: SQLite,
    name: String,
}

impl FileDataSource {
    pub fn read(&self) -> Option<Vec<String>> {
        let query = format!("SELECT path FROM {}", self.name);
        let mut stmt = self.sqlite.conn.prepare(&query).ok()?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).ok()?;
        Some(rows.filter_map(Result::ok).collect())
    }

    pub fn cache_file_search_paths(&mut self) -> Result<(), DSError> {
        println!("Starting to cache application paths...");
        let start = Instant::now();
        let paths = Arc::new(Mutex::new(Vec::new()));

        if let Some(directories) = AppConfig::new().get_search_directories() {
            let mut walker = WalkBuilder::new(directories.first().unwrap());
            directories
                .iter()
                .skip(1)
                .fold(&mut walker, |builder, dir| builder.add(dir))
                .threads(cmp::min(4, num_cpus::get()))
                .hidden(false)
                .max_depth(Some(6))
                .build_parallel()
                .run(|| {
                    let paths = Arc::clone(&paths);
                    Box::new(move |entry| {
                        use ignore::WalkState;

                        if let Ok(entry) = entry {
                            let path = entry.path().to_owned();

                            #[cfg(target_os = "macos")]
                            {
                                if path.extension().map_or(false, |ext| ext == "app") {
                                    paths.lock().unwrap().push(path);
                                    return WalkState::Continue;
                                }
                                let path_str = path.to_string_lossy();
                                if path_str.ends_with("/Contents")
                                    || path_str.contains("Native Instruments")
                                    || path_str.contains("Adobe Creative Cloud")
                                {
                                    return WalkState::Skip;
                                }
                                if !path.is_dir() && !path.is_symlink() && path.is_absolute() {
                                    paths.lock().unwrap().push(path);
                                    return WalkState::Continue;
                                }
                            }

                            #[cfg(target_os = "windows")]
                            {
                                if path.extension().map_or(false, |ext| ext == "exe") {
                                    paths.lock().unwrap().push(path);
                                }
                            }

                            // this probably doesn't work? AI wrote it. /shrug.
                            #[cfg(target_os = "linux")]
                            {
                                if path.extension().map_or(false, |ext| ext == "desktop") {
                                    paths.lock().unwrap().push(path);
                                }
                            }
                        }
                        WalkState::Continue
                    })
                });

            let paths = paths.lock().unwrap();

            // Clear existing entries and insert new ones (this is a complete reindex)
            let transaction_handle = self.sqlite.conn.transaction()?;
            transaction_handle.execute(&format!("DELETE FROM {}", self.name), [])?;
            let utc: DateTime<Utc> = Utc::now();

            for path in paths.iter() {
                transaction_handle.execute(
                    &format!(
                        "INSERT OR IGNORE INTO {} (path, last_updated) VALUES (?1, ?2)",
                        self.name
                    ),
                    params![path.to_string_lossy().to_string(), format!("{:?}", utc)],
                )?;
            }
            if let Err(error) = transaction_handle.commit() {
                Err(DSError::TransactionError(error))
            } else {
                println!(
                    "Finished caching {} application paths in {}ms",
                    paths.len(),
                    start.elapsed().as_millis()
                );

                Ok(())
            }
        } else {
            Err(DSError::MissingSearchDir)
        }
    }
}

impl DataSource<Vec<FileInfo>> for FileDataSource {
    fn new(name: &str) -> Self {
        if let Ok(sqlite) = SQLite::new(name, false) {
            let transaction = format!(
                "CREATE TABLE IF NOT EXISTS {} (
              id INTEGER PRIMARY KEY,
              path TEXT NOT NULL UNIQUE,
              last_updated TEXT NOT NULL
            )",
                name
            );
            if let Err(e) = sqlite.conn.execute(&transaction, []) {
                println!(
                    "failed to create the table '{}', maybe it already exists?\n{:?}",
                    name, e
                )
            };
            Self {
                sqlite,
                name: name.to_string(),
            }
        } else {
            panic!("Error initializing the FileDataSource")
        }
    }

    fn update_cache(&mut self) {
        if let Err(e) = self.cache_file_search_paths() {
            eprintln!("Error updating the file search cache! {:?}", e);
        }
        cache_all_app_icons();
    }

    fn query(&self, query: &Query) -> Option<Vec<FileInfo>> {
        match self.read() {
            None => None,
            Some(dirs) => score_files(query, dirs).and_then(|filepaths| {
                filepaths
                    .iter()
                    .map(|filepath| FileInfo::from_string(filepath.clone()))
                    .take(50)
                    .collect()
            }),
        }
    }
}
