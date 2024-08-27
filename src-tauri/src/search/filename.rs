use crate::settings::AppConfig;
use crate::utilities::{cache_all_app_icons, cache_app_icon_path};
use std::str::FromStr;

use ignore::WalkBuilder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::{
    ffi::{OsStr, OsString},
    fs,
    path::Path,
    time::Instant,
};
use swordfish_types::{DataSource, Query, QueryResult};

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

impl AsRef<str> for FileType {
    fn as_ref(&self) -> &str {
        match self {
            FileType::File => "file",
            FileType::Directory => "directory",
            FileType::Binary => "binary",
            FileType::Typescript => "typescript",
            FileType::Javascript => "javascript",
            FileType::Rust => "rust",
            FileType::Python => "python",
            FileType::C => "c",
            FileType::Cpp => "cpp",
            FileType::Java => "java",
            FileType::Go => "go",
            FileType::UnixShell => "unixShell",
            FileType::WindowsShell => "windowsShell",
            FileType::Text => "text",
            FileType::Markdown => "markdown",
            FileType::Json => "json",
            FileType::Xml => "xml",
            FileType::Yaml => "yaml",
            FileType::Toml => "toml",
            FileType::Sql => "sql",
            FileType::Html => "html",
            FileType::Css => "css",
            FileType::Sass => "sass",
            FileType::Application => "application",
            FileType::Image => "image",
            FileType::Video => "video",
            FileType::Audio => "audio",
            FileType::Archive => "archive",
            FileType::Pdf => "pdf",
            FileType::Word => "word",
            FileType::Excel => "excel",
            FileType::Powerpoint => "powerpoint",
            FileType::CSV => "cSV",
            FileType::Other => "other",
        }
    }
}

impl FromStr for FileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file" => Ok(FileType::File),
            "directory" => Ok(FileType::Directory),
            "binary" => Ok(FileType::Binary),
            "typescript" => Ok(FileType::Typescript),
            "javascript" => Ok(FileType::Javascript),
            "rust" => Ok(FileType::Rust),
            "python" => Ok(FileType::Python),
            "c" => Ok(FileType::C),
            "cpp" => Ok(FileType::Cpp),
            "java" => Ok(FileType::Java),
            "go" => Ok(FileType::Go),
            "unixShell" => Ok(FileType::UnixShell),
            "windowsShell" => Ok(FileType::WindowsShell),
            "text" => Ok(FileType::Text),
            "markdown" => Ok(FileType::Markdown),
            "json" => Ok(FileType::Json),
            "xml" => Ok(FileType::Xml),
            "yaml" => Ok(FileType::Yaml),
            "toml" => Ok(FileType::Toml),
            "sql" => Ok(FileType::Sql),
            "html" => Ok(FileType::Html),
            "css" => Ok(FileType::Css),
            "sass" => Ok(FileType::Sass),
            "application" => Ok(FileType::Application),
            "image" => Ok(FileType::Image),
            "video" => Ok(FileType::Video),
            "audio" => Ok(FileType::Audio),
            "archive" => Ok(FileType::Archive),
            "pdf" => Ok(FileType::Pdf),
            "word" => Ok(FileType::Word),
            "excel" => Ok(FileType::Excel),
            "powerpoint" => Ok(FileType::Powerpoint),
            "cSV" => Ok(FileType::CSV),
            "other" => Ok(FileType::Other),
            _ => Err(()),
        }
    }
}

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
            .and_then(|_fname| {
                let _path_str = path.to_string_lossy().to_string();
                let file_name = path.file_stem().and_then(OsStr::to_str).map(str::to_string);

                let extension = path.extension().and_then(OsStr::to_str).map(str::to_string);

                if let Ok(metadata) = fs::metadata(path) {
                    let file_type = if extension.is_some() {
                        get_filetype_from_extension(extension.as_deref())
                    } else if metadata.is_dir() {
                        FileType::Directory
                    } else {
                        FileType::Other
                    };
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
                        file_name,
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
}

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
