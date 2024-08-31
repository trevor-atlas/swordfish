use crate::settings::AppConfig;
use crate::utilities::{cache_all_app_icons, cache_app_icon_path};
use fuzzy_matcher::skim::SkimMatcherV2;
use ignore::WalkBuilder;
use std::cmp;
use std::{ffi::OsString, time::Instant};
use swordfish_types::{DataSource, FileInfo, Query};

use crossbeam_channel;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const LIMIT: usize = 300;
#[derive(Serialize, Deserialize)]
struct AppCache {
    paths: HashSet<PathBuf>,
}

pub fn cache_application_paths() {
    let start = Instant::now();
    println!("Starting to cache application paths...");

    let cache_file = "app_paths_cache.json";
    let paths = Arc::new(Mutex::new(HashSet::new()));

    #[cfg(target_os = "macos")]
    let directories = ["/Applications", "/System/Applications"];

    #[cfg(target_os = "windows")]
    let directories = ["C:\\Program Files", "C:\\Program Files (x86)"];

    #[cfg(target_os = "linux")]
    let directories = ["/usr/share/applications", "/usr/local/share/applications"];

    let mut walker = WalkBuilder::new(directories[0]);

    directories
        .iter()
        .skip(1)
        .fold(&mut walker, |builder, dir| builder.add(dir))
        .threads(cmp::min(2, num_cpus::get()))
        .hidden(true)
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
                            paths.lock().unwrap().insert(path);
                            return WalkState::Continue;
                        }
                        if path.to_string_lossy().ends_with("/Contents") {
                            return WalkState::Skip;
                        }
                    }

                    #[cfg(target_os = "windows")]
                    {
                        if path.extension().map_or(false, |ext| ext == "exe") {
                            paths.lock().unwrap().insert(path);
                        }
                    }

                    #[cfg(target_os = "linux")]
                    {
                        if path.extension().map_or(false, |ext| ext == "desktop") {
                            paths.lock().unwrap().insert(path);
                        }
                    }
                }
                WalkState::Continue
            })
        });

    let app_cache = AppCache {
        paths: paths.lock().unwrap().clone(),
    };

    let file = File::create(cache_file).expect("Failed to create cache file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &app_cache).expect("Failed to write cache to file");

    println!(
        "Finished caching {} application paths in {}ms",
        app_cache.paths.len(),
        start.elapsed().as_millis()
    );
}

pub fn search(query: &Query, directories: Vec<String>) -> Option<Vec<FileInfo>> {
    let search_string = query.search_string.clone();
    let (tx, rx) = crossbeam_channel::unbounded::<(i64, String)>();
    let start = Instant::now();
    let mut walker = WalkBuilder::new(directories.first()?);

    directories
        .iter()
        .skip(1)
        .fold(&mut walker, |builder, dir| builder.add(dir))
        .threads(cmp::min(6, num_cpus::get()))
        .hidden(true)
        .build_parallel()
        .run(|| {
            let case_sensitive = search_string.chars().any(|c| c.is_uppercase());
            let search = search_string.clone();
            let matcher = SkimMatcherV2::default();
            let tx = tx.clone();
            let mut counter: usize = 0;
            Box::new(move |path_entry| {
                use ignore::WalkState;
                if counter >= LIMIT {
                    return WalkState::Quit;
                }

                if let Ok(entry) = path_entry {
                    let path = entry.path();

                    let is_app = path.extension() == Some(&OsString::from("app"))
                        && !entry.file_name().is_empty();
                    let path_str = path.to_string_lossy().to_string();

                    #[cfg(target_os = "macos")]
                    {
                        if is_app {
                            let app_path = path;
                            cache_app_icon_path(
                                app_path.to_string_lossy().to_string().as_str(),
                                app_path
                                    .file_stem()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string()
                                    .as_str(),
                            );
                        }

                        // mac apps are technically directories, so we need to make sure we don't
                        // search inside them.
                        if !is_app
                            && (path.starts_with("/Applications/")
                                || path.starts_with("/System/Applications/"))
                            && path_str.ends_with("/Contents")
                        {
                            return WalkState::Skip;
                        }
                    }

                    let mut score = matcher
                        .fuzzy(&path_str, &search, true)
                        .map(|res| res.0)
                        .unwrap_or(0);

                    if let Some(fname) = path.file_name() {
                        let fname_score = matcher
                            .fuzzy(fname.to_string_lossy().to_string().as_str(), &search, true)
                            .map(|res| res.0)
                            .unwrap_or(0);
                        score += fname_score;
                    }

                    if is_app && score > 0 {
                        // bumping app matches because they're more likely to be relevant
                        score += 10;
                    }

                    if score > 0
                        && (is_app || !path.is_dir())
                        && tx.send((score, path.to_string_lossy().to_string())).is_ok()
                    {
                        counter += 1;
                        return WalkState::Continue;
                    }
                }
                WalkState::Continue
            })
        });

    drop(tx);
    println!("finished search in {}ms", start.elapsed().as_millis());
    let mut directories: Vec<(i64, String)> = rx.iter().collect();

    directories.sort_by(|a, b| b.0.cmp(&a.0));

    let ordered: Vec<FileInfo> = directories
        .iter()
        .map(|result| FileInfo::from_string(result.1.clone()))
        .flat_map(|x| x)
        .take(100)
        .collect();

    Some(ordered)
}

pub struct FileDataSource {
    pub last_updated: u64,
}

impl DataSource<Vec<FileInfo>> for FileDataSource {
    fn new() -> Self {
        Self { last_updated: 0 }
    }

    fn update_cache() {
        cache_all_app_icons();
    }

    fn query(&self, query: &Query) -> Option<Vec<FileInfo>> {
        let settings = AppConfig::new();
        match settings.get_search_directories() {
            None => None,
            Some(dirs) => search(query, dirs),
        }
    }
}
