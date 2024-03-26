use crate::query::Query;
use crate::settings::AppConfig;

use ignore::WalkBuilder;
use regex::Regex;
use std::cmp;
use std::fs;
use tauri::api::file;

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

fn get_filetype_from_filename(file_extension: Option<&str>) -> FileType {
    match file_extension {
        Some(ext) => match ext {
            "ts" | "tsx" => FileType::Typescript,
            "js" | "jsx" => FileType::Javascript,
            "rs" => FileType::Rust,
            "py" => FileType::Python,
            "c" => FileType::C,
            "cpp" => FileType::Cpp,
            "java" => FileType::Java,
            "go" => FileType::Go,
            "txt" => FileType::Text,
            "md" => FileType::Markdown,
            "json" => FileType::Json,
            "xml" => FileType::Xml,
            "yaml" => FileType::Yaml,
            "toml" => FileType::Toml,
            "sql" => FileType::Sql,
            "html" => FileType::Html,
            "css" => FileType::Css,
            "sass" => FileType::Sass,
            "exe" => FileType::Application,
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" => FileType::Image,
            "mp4" | "mkv" | "avi" | "mov" | "flv" | "wmv" | "webm" => FileType::Video,
            "mp3" | "wav" | "flac" | "ogg" | "m4a" | "wma" => FileType::Audio,
            "zip" | "tar" | "gz" | "7z" | "rar" | "iso" | "dmg" => FileType::Archive,
            "pdf" => FileType::Pdf,
            "doc" | "docx" => FileType::Word,
            "xls" | "xlsx" => FileType::Excel,
            "ppt" | "pptx" => FileType::Powerpoint,
            "csv" => FileType::CSV,
            "app" | "exe" => FileType::Application,
            "sh" => FileType::UnixShell,
            "bat" => FileType::WindowsShell,
            "" => FileType::Directory,
            _ => FileType::File,
        },
        None => FileType::File,
    }
}

const LIMIT: usize = 100;

pub enum FileType {
    File,
    Directory,
    Typescript,
    Javascript,
    Rust,
    Python,
    C,
    Cpp,
    Java,
    Go,
    UnixShell,
    WindowsShell,
    Text,
    Markdown,
    Json,
    Xml,
    Yaml,
    Toml,
    Sql,
    Html,
    Css,
    Sass,
    Application,
    Image,
    Video,
    Audio,
    Archive,
    Pdf,
    Word,
    Excel,
    Powerpoint,
    CSV,
}
pub struct FileInfo {
    pub file_name: String,
    pub path: String,
    pub extension: String,
    pub file_type: FileType,
    pub size: u64,
    pub last_modified: u64,
    pub created: u64,
}

impl FileInfo {
    pub fn new() -> Self {
        Self {
            file_name: "".to_string(),
            path: "".to_string(),
            extension: "".to_string(),
            file_type: FileType::File,
            size: 0,
            last_modified: 0,
            created: 0,
        }
    }

    pub fn from_str(filepath: &str) -> Self {
        let path = Path::new(filepath);
        let file_name = path.to_string_lossy().to_string();
        let extension = get_extension_from_filename(&file_name);
        let file_type = get_filetype_from_filename(extension.as_deref());

        match fs::metadata(path) {
            Ok(metadata) => {
                let size = metadata.len();
                let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
                let created = metadata.created().unwrap().elapsed().unwrap().as_secs();
                Self {
                    file_name,
                    path: filepath.to_string(),
                    extension: extension.unwrap_or("".to_string()),
                    file_type,
                    size,
                    last_modified,
                    created,
                }
            }
            Err(_) => Self {
                file_name,
                path: filepath.to_string(),
                extension: extension.unwrap_or("".to_string()),
                file_type,
                size: 0,
                last_modified: 0,
                created: 0,
            },
        }
    }

    pub fn from_string(filepath: String) -> Self {
        Self::from_str(&filepath)
    }

    // pub fn from_path(path: Path) -> Self {
    //     Self::from_str(path.to_str().unwrap())
    // }
}

pub fn search(query: &Query) -> Option<Vec<FileInfo>> {
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
                .threads(cmp::min(6, num_cpus::get()))
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
                                let file_name = file_name.to_string_lossy().to_string();

                                #[cfg(target_os = "macos")]
                                {
                                    // MacOS .app files are technically directories
                                    // so we need to check if the file_name ends with .app
                                    // and skip decending into that directory
                                    if file_name.ends_with(".app") {
                                        if reg_exp.is_match(&file_name) {
                                            if tx.send(path.to_string_lossy().to_string()).is_ok() {
                                                counter += 1;
                                            }
                                        }
                                        return WalkState::Skip;
                                    }
                                }

                                if reg_exp.is_match(&file_name) {
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
                    .collect(),
            );
        }
        None => {
            return None;
        }
    }
}
