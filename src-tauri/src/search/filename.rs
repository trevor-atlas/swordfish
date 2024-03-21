use crate::settings::AppConfig;
use ignore::DirEntry;
use ignore::WalkBuilder;
use regex::Regex;
use std::cmp;
use std::fs;
use std::io::Write;
use std::time::Instant;

const FUZZY_SEARCH: &str = r".*";

pub fn build_regex_search_input(
    search_input: Option<&str>,
    file_ext: Option<&str>,
    strict: bool,
    ignore_case: bool,
) -> Regex {
    let file_type = file_ext.unwrap_or("*");
    let search_input = search_input.unwrap_or(r"\w+");

    let mut formatted_search_input = if strict {
        format!(r#"{search_input}\.{file_type}$"#)
    } else {
        format!(r#"{search_input}{FUZZY_SEARCH}\.{file_type}$"#)
    };

    if ignore_case {
        formatted_search_input = set_case_insensitive(&formatted_search_input);
    }
    Regex::new(&formatted_search_input).unwrap()
}

fn set_case_insensitive(formatted_search_input: &str) -> String {
    "(?i)".to_owned() + formatted_search_input
}

pub fn search() {
    let regex_search_input = build_regex_search_input(Some("POE-T"), None, false, false);
    let (tx, rx) = crossbeam_channel::unbounded::<String>();
    let settings = AppConfig::new();
    let start = Instant::now();
    if let Some(dirs) = settings.get_search_directories() {
        if dirs.is_empty() {
            return;
        }
        let stdout_thread = std::thread::spawn(move || {
            let s: Vec<u8> = rx
                .iter()
                .map(|dent| dent.as_bytes().to_owned())
                .flatten()
                .collect();
            fs::write("C:\\Users\\Trevor\\Desktop\\swordfish-output", s)
                .expect("Unable to write file");
            println!("Ran search in: {:?} ms", start.elapsed());
        });

        let mut walker = WalkBuilder::new(dirs[0].clone());
        dirs.iter()
            .skip(1)
            .fold(&mut walker, |builder, dir| builder.add(dir))
            .threads(cmp::min(12, num_cpus::get()))
            .build_parallel()
            .run(|| {
                let tx = tx.clone();
                let reg_exp: Regex = regex_search_input.clone();
                let mut counter = 0;
                let limit = 100;

                Box::new(move |path_entry| {
                    use ignore::WalkState;
                    if let Ok(entry) = path_entry {
                        let path = entry.path();
                        if let Some(file_name) = path.file_name() {
                            // Lossy means that if the file name is not valid UTF-8
                            // it will be replaced with �.
                            // Will return the file name with extension.
                            let file_name = file_name.to_string_lossy().to_string();
                            if reg_exp.is_match(&file_name) {
                                // Continue searching if the send was successful
                                // and there is no limit or the limit has not been reached
                                if tx.send(path.display().to_string()).is_ok() && (counter < limit)
                                {
                                    counter += 1;
                                    return WalkState::Continue;
                                }

                                return WalkState::Quit;
                            }
                        }
                    }
                    WalkState::Continue
                })
            });

        drop(tx);
        stdout_thread.join().unwrap();
        // let v: Vec<_> = rx.iter().collect();
        // v.iter()
        //     .for_each(|d| println!("{}", d.path().to_str().unwrap()));
    }
}
