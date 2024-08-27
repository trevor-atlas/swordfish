use crate::settings::AppConfig;
use crate::utilities::{cache_all_app_icons, cache_app_icon_path};
use ignore::WalkBuilder;
use regex::Regex;
use std::cmp;
use std::{ffi::OsString, time::Instant};
use swordfish_types::{DataSource, FileInfo, Query};

const LIMIT: usize = 100;

pub fn search(query: &Query, directories: Vec<String>) -> Option<Vec<FileInfo>> {
    let mut search_string = query.search_string.clone();

    if !search_string.chars().any(|c| c.is_uppercase()) {
        search_string = format!("(?i){}", search_string);
    }
    let regex_search_input = match Regex::new(&search_string) {
        Ok(regex) => regex,
        Err(_e) => return None,
    };
    let (tx, rx) = crossbeam_channel::unbounded::<String>();
    let start = Instant::now();

    let mut walker = WalkBuilder::new(directories.get(0)?.clone());
    directories
        .iter()
        .skip(1)
        .fold(&mut walker, |builder, dir| builder.add(dir))
        .threads(cmp::min(6, num_cpus::get()))
        .hidden(true)
        .build_parallel()
        .run(|| {
            let tx = tx.clone();
            let reg_exp: Regex = regex_search_input.clone();
            let mut counter: usize = 0;
            Box::new(move |path_entry| {
                use ignore::WalkState;
                if counter >= LIMIT {
                    return WalkState::Quit;
                }

                if let Ok(entry) = path_entry {
                    let path = entry.path();

                    #[cfg(target_os = "macos")]
                    {
                        if path.extension() == Some(&OsString::from("app"))
                            && path.file_name().is_some()
                        {
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

                        let path_str = path.to_string_lossy().to_string();
                        if (path.starts_with("/Applications/")
                            || path.starts_with("/System/Applications/"))
                            && path_str.ends_with("/Contents")
                        {
                            return WalkState::Skip;
                        }
                    }

                    if let Some(file_name) = path.file_name() {
                        // Lossy means that if the file name is not valid UTF-8
                        // it will be replaced with �.
                        let fname = file_name.to_string_lossy().to_string();

                        if reg_exp.is_match(&fname) {
                            if tx.send(path.to_string_lossy().to_string()).is_ok() {
                                counter += 1;
                                return WalkState::Continue;
                            }
                        }
                    }
                }
                WalkState::Continue
            })
        });

    drop(tx);
    println!("finished search in {}ms", start.elapsed().as_millis());
    return Some(
        rx.iter()
            .map(|file_path| FileInfo::from_string(file_path))
            .flat_map(|x| x)
            .collect(),
    );
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
