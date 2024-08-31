use crate::settings::AppConfig;
use crate::utilities::{cache_all_app_icons, cache_app_icon_path, get_data_path};
use chrono::prelude::DateTime;
use chrono::Utc;
use dirs::data_dir;
use fuzzy_matcher::skim::SkimMatcherV2;
use ignore::WalkBuilder;
use rusqlite::config::DbConfig;
use std::{cmp, fs, path};
use std::{ffi::OsString, time::Instant};
use swordfish_types::{DataSource, FileInfo, Query};
use thiserror::Error;

use crossbeam_channel;
use rusqlite::{params, Connection, DatabaseName, Result, MAIN_DB};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

const LIMIT: usize = 300;

pub struct SQLite {
    conn: Connection,
}

impl SQLite {
    pub fn new(p: &mut PathBuf, database_name: &str, readonly: bool) -> Result<Self, &'static str> {
        p.push(&format!("{}.sqlite", &database_name));
        let mut conn = match Connection::open(p) {
            Ok(conn) => conn,
            Err(_) => {
                return Err("Couldn't create the db connection");
            }
        };
        match conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                  id INTEGER PRIMARY KEY,
                  path TEXT NOT NULL UNIQUE,
                  last_updated TEXT NOT NULL
                )",
                database_name
            ),
            [],
        ) {
            Ok(conn) => {}
            Err(e) => {
                println!(
                    "failed to create the table '{}', maybe it already exists?\n{:?}",
                    database_name, e
                )
            }
        }
        let db = Some(MAIN_DB);

        // enables write-ahead log so that your reads do not block writes and vice-versa.
        conn.pragma_update(db, "journal_mode", "WAL").ok();

        // sqlite will wait 5 seconds to obtain a lock before returning SQLITE_BUSY errors, which will significantly reduce them.
        conn.pragma_update(db, "busy_timeout", 5000).ok();

        // sqlite will sync less frequently and be more performant, still safe to use because of the enabled WAL mode.
        conn.pragma_update(db, "synchronous", "NORMAL").ok();

        // negative number means kilobytes, in this case 20MB of memory for cache.
        conn.pragma_update(db, "cache_size", -20000).ok();

        // because of historical reasons foreign keys are disabled by default, we should manually enable them.
        conn.pragma_update(db, "foreign_keys", true).ok();

        // moves temporary tables from disk into RAM, speeds up performance a lot.
        conn.pragma_update(db, "temp_store", "memory").ok();

        // Do NOT use cache=shared! Some tutorials recommend configuring it, but this is how you get nasty SQLITE_BUSY errors. It is disabled by default, so you don't have to do anything extra.
        // If you know that transaction can possibly do a write, always use BEGIN IMMEDIATE or you can a get SQLITE_BUSY error. Check your framework,  you should be able to set this at the connection level.

        if readonly {
            conn.pragma_update(db, "mode", "ro").ok();
        } else {
            conn.set_transaction_behavior(rusqlite::TransactionBehavior::Immediate);
            conn.pragma_update(db, "txlock", "IMMEDIATE").ok();
            conn.pragma_update(db, "mode", "rwc").ok();
        }

        Ok(Self { conn })
    }
}

pub struct SFCache {
    cache: SQLite,
}

#[derive(Error, Debug)]
pub enum SFCacheError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to locate data directory")]
    MissingDir(&'static str),
    #[error("Failed to load search directories from settings")]
    MissingSearchDir,
    #[error("Transaction Error")]
    TransactionError(#[from] rusqlite::Error),
}

impl SFCache {
    pub fn new() -> Result<Self, &'static str> {
        if let Some(mut dir) = get_data_path() {
            Ok(Self {
                cache: SQLite::new(&mut dir, "sf_cache", false)?,
            })
        } else {
            Err("Couldn't locate data directory to create a cache db")
        }
    }

    pub fn cache_application_paths(&mut self) -> Result<(), SFCacheError> {
        let start = Instant::now();
        println!("Starting to cache application paths...");

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

                            // this probably doesn't work? AI wrote it. /shrug
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

            // Clear existing entries and insert new ones
            let transaction_handle = self.cache.conn.transaction()?;
            transaction_handle.execute("DELETE FROM sf_cache", [])?;
            let utc: DateTime<Utc> = Utc::now();

            for path in paths.iter() {
                transaction_handle.execute(
                    "INSERT OR IGNORE INTO sf_cache (path, last_updated) VALUES (?1, ?2)",
                    params![path.to_string_lossy().to_string(), format!("{:?}", utc)],
                )?;
            }
            if let Err(error) = transaction_handle.commit() {
                Err(SFCacheError::TransactionError(error))
            } else {
                println!(
                    "Finished caching {} application paths in {}ms",
                    paths.len(),
                    start.elapsed().as_millis()
                );

                Ok(())
            }
        } else {
            Err(SFCacheError::MissingSearchDir)
        }
    }

    pub fn get_cached_appliation_paths(&self) -> Result<Vec<String>> {
        let mut stmt = self.cache.conn.prepare("SELECT path FROM sf_cache")?;
        let paths = stmt.query_map([], |row| {
            let path_str: String = row.get(0)?;
            Ok(path_str)
        })?;

        paths.collect()
    }
}

pub fn search(query: &Query, directories: Vec<String>) -> Option<Vec<FileInfo>> {
    let search_string = query.search_string.clone();
    let start = Instant::now();
    let matcher = SkimMatcherV2::default();
    let mut scored_directories: Vec<(i64, String)> = directories
        .iter()
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
        .collect();
    println!("finished search in {}ms", start.elapsed().as_millis());

    scored_directories.sort_by(|a, b| b.0.cmp(&a.0));

    Some(
        scored_directories
            .iter()
            .flat_map(|result| FileInfo::from_string(result.1.clone()))
            .take(50)
            .collect(),
    )
}

pub struct FileDataSource {
    cache: SFCache,
}

impl DataSource<Vec<FileInfo>> for FileDataSource {
    fn new() -> Self {
        if let Ok(cache) = SFCache::new() {
            Self { cache }
        } else {
            panic!("Couldn't create a cache instance")
        }
    }

    fn update_cache(&mut self) {
        if self.cache.cache_application_paths().is_err() {
            eprintln!("failed to cache search paths");
        };
        cache_all_app_icons();
    }

    fn query(&self, query: &Query) -> Option<Vec<FileInfo>> {
        match self.cache.get_cached_appliation_paths() {
            Err(_) => None,
            Ok(dirs) => search(query, dirs),
        }
    }
}
