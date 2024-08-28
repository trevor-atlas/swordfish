use crate::settings::AppConfig;
use crate::utilities::{cache_all_app_icons, cache_app_icon_path};
use fuzzy_matcher::skim::SkimMatcherV2;
use ignore::WalkBuilder;
use std::cmp;
use std::{ffi::OsString, time::Instant};
use swordfish_types::{DataSource, FileInfo, Query};

const LIMIT: usize = 300;

pub fn search(query: &Query, directories: Vec<String>) -> Option<Vec<FileInfo>> {
    let search_string = query.search_string.clone();
    let (tx, rx) = crossbeam_channel::unbounded::<(i64, String)>();
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
            let case_sensitive = search_string.chars().any(|c| c.is_uppercase());
            let search = search_string.clone();
            // let matcher = if case_sensitive {
            //     SkimMatcherV2::default()
            // } else {
            //     SkimMatcherV2::default().ignore_case()
            // };

            let matcher = SkimMatcherV2::default().ignore_case();
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

                        let path_str = path.to_string_lossy().to_string();
                        if !is_app
                            && (path.starts_with("/Applications/")
                                || path.starts_with("/System/Applications/"))
                            && path_str.ends_with("/Contents")
                        {
                            return WalkState::Skip;
                        }
                    }

                    let path_str = if case_sensitive {
                        path.to_string_lossy().to_string()
                    } else {
                        path.to_string_lossy().to_string().to_lowercase()
                    };

                    let mut score = matcher
                        .fuzzy(&path_str, &search, true)
                        .map(|res| res.0)
                        .unwrap_or(0);

                    match path.file_name() {
                        Some(fname) => {
                            let file_name_str = if case_sensitive {
                                fname.to_string_lossy().to_string()
                            } else {
                                fname.to_string_lossy().to_string().to_lowercase()
                            };
                            let fname_score = matcher
                                .fuzzy(&file_name_str, &search, true)
                                .map(|res| res.0)
                                .unwrap_or(0);
                            score += fname_score;
                        }
                        None => {}
                    }

                    if is_app && score > 0 {
                        score += 10;
                    }

                    if score > 0 && (is_app || !path.is_dir()) {
                        if tx.send((score, path.to_string_lossy().to_string())).is_ok() {
                            counter += 1;
                            return WalkState::Continue;
                        }
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
