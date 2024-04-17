use crate::query::Query;
use crate::settings::AppConfig;
use crate::utilities::cache_app_icon_path;

use ignore::WalkBuilder;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::cmp;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use std::path::Path;
use std::thread;
use std::time::Instant;

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

fn get_filetype_from_extension(file_extension: Option<&str>) -> FileType {
    match file_extension.unwrap_or("") {
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
        "" => FileType::Other,
        _ => FileType::Other,
    }
}

const LIMIT: usize = 100;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileType {
    File,
    Directory,
    Binary,
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
    Other,
}
pub struct FileInfo {
    pub file_name: Option<String>,
    pub path: String,
    pub extension: Option<String>,
    pub file_type: FileType,
    pub size: u64,
    pub last_modified: Option<u64>,
    pub created: Option<u64>,
}

impl FileInfo {
    pub fn from_str(filepath: &str) -> Option<Self> {
        let path = Path::new(filepath);
        path.file_name()
            .and_then(|fname| Some(fname.to_string_lossy().to_string()))
            .and_then(|fname| {
                let path_str = path.to_string_lossy().to_string();
                let file_name = path
                    .file_name()
                    .and_then(OsStr::to_str)
                    .and_then(|str| Some(str.to_string()))
                    .and_then(|a| {
                        let parts = a.split('.').next();
                        if parts.is_some() {
                            Some(parts.unwrap().to_string())
                        } else {
                            None
                        }
                    })
                    .map(|s| s.to_string());

                let extension = path
                    .extension()
                    .and_then(OsStr::to_str)
                    .and_then(|str| Some(str.to_string()));
                let file_type = if extension.is_some() {
                    get_filetype_from_extension(extension.as_deref())
                } else {
                    FileType::Other
                };

                if let Ok(metadata) = fs::metadata(path) {
                    let size = metadata.len();
                    let last_modified = metadata
                        .modified()
                        .ok()
                        .and_then(|date| Some(date.elapsed()))
                        .and_then(|date| date.ok().and_then(|d| Some(d.as_secs())));
                    let created = metadata
                        .created()
                        .ok()
                        .and_then(|date| Some(date.elapsed()))
                        .and_then(|date| date.ok().and_then(|d| Some(d.as_secs())));
                    return Some(Self {
                        file_name: file_name,
                        path: filepath.to_string(),
                        extension,
                        file_type,
                        size,
                        last_modified,
                        created,
                    });
                }
                None
            })
    }

    pub fn from_string(filepath: String) -> Option<Self> {
        match Self::from_str(&filepath) {
            Some(file_info) => Some(file_info),
            None => None,
        }
    }

    pub fn from_pathbuf(path: PathBuf) -> Option<Self> {
        match path.to_str() {
            Some(path_str) => Self::from_str(path_str),
            None => None,
        }
    }
}

pub fn search(query: &Query) -> Option<Vec<FileInfo>> {
    let mut search_string = query.search_string.clone();

    if !search_string.chars().any(|c| c.is_uppercase()) {
        search_string = format!("(?i){}", search_string);
    }
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
        None => {
            return None;
        }
    }
}
