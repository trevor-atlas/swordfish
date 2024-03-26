use crate::query::Query;
use crate::settings::AppConfig;

use ignore::WalkBuilder;
use regex::Regex;
use std::cmp;
use std::fs;

use std::path::Path;
use std::time::Instant;

const FUZZY_SEARCH: &str = r".*";

fn get_extension_from_filename(filename: &str) -> Option<String> {
    // Change it to a canonical file path.
    match Path::new(&filename).canonicalize() {
        Ok(path) => Some(
            path.to_str()?
                .split('/')
                .map(|str| str.to_string())
                .collect::<Vec<String>>()
                .last()?
                .split('.')
                .last()?
                .to_string(),
        ),
        Err(e) => None,
    }
}

const LIMIT: usize = 100;

pub fn search(query: &Query) -> Option<Vec<String>> {
    let mut search_string = query.search_string.clone();

    if !search_string.chars().any(|c| c.is_uppercase()) {
        search_string = format!("(?i){}", search_string);
    }
    println!("search_string: {}", search_string);
    let regex_search_input = match Regex::new(&search_string) {
        Ok(regex) => regex,
        Err(e) => return None,
    };
    let (tx, rx) = crossbeam_channel::unbounded::<String>();
    let settings = AppConfig::new();
    let start = Instant::now();
    match settings.get_search_directories() {
        Some(search_dirs) => {
            let mut walker = WalkBuilder::new(search_dirs.get(0)?.clone());
            search_dirs
                .iter()
                .skip(1)
                .fold(&mut walker, |builder, dir| builder.add(dir))
                .threads(cmp::min(6, num_cpus::get() / 2))
                .build_parallel()
                .run(|| {
                    let tx = tx.clone();
                    let reg_exp: Regex = regex_search_input.clone();
                    let mut counter: usize = 0;
                    let str = search_string.clone();
                    Box::new(move |path_entry| {
                        use ignore::WalkState;
                        if counter >= LIMIT {
                            return WalkState::Quit;
                        }
                        if let Ok(entry) = path_entry {
                            let path = entry.path();
                            if let Some(file_name) = path.file_name() {
                                // Lossy means that if the file name is not valid UTF-8
                                // it will be replaced with �.
                                // Will return the file name with extension.
                                let file_name = file_name.to_string_lossy().to_string();

                                #[cfg(target_os = "macos")]
                                {
                                    if file_name.ends_with(".app") {
                                        if reg_exp.is_match(&file_name) {
                                            println!(
                                                "Match file_name: {}\n query: {}",
                                                file_name, str
                                            );
                                            if tx.send(path.display().to_string()).is_ok() {
                                                counter += 1;
                                            }
                                        }
                                        return WalkState::Skip;
                                    }
                                }

                                if reg_exp.is_match(&file_name) {
                                    if tx.send(path.display().to_string()).is_ok() {
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
            return Some(rx.iter().collect());
        }
        None => {
            return None;
        }
    }
}
