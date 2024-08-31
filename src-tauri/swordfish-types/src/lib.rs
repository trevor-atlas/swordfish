use core::fmt;
use std::{ffi::OsStr, fs, path::Path, str::FromStr};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub trait DataSource<T> {
    fn new() -> Self;
    fn update_cache(&mut self);
    fn query(&self, query: &Query) -> Option<T>;
}

#[derive(TS, Deserialize, Debug, Serialize, Clone)]
#[ts(export, export_to = "../../../src/types/")]
pub enum QueryMode {
    Search,
    BrowserHistory,
    Chat,
    Scripts,
}

#[derive(TS, Deserialize, Debug, Serialize, Clone)]
#[ts(export, export_to = "../../../src/types/")]
pub struct Query {
    pub search_string: String,
    pub mode: QueryMode,
}

#[derive(TS, Deserialize, Debug, Serialize, Clone)]
#[ts(export, export_to = "../../../src/types/")]
pub enum QueryResultType {
    File,
    BrowserHistory,
    Script,
    Action,
    Calculator,
}

#[derive(TS, Deserialize, Debug, Serialize, Clone)]
#[ts(export, export_to = "../../../src/types/")]
pub struct QueryResultItem {
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    pub heading: String,
    pub subheading: String,
    pub value: String,
    pub preview: Option<ResultPreview>,
    #[serde(rename = "type")]
    pub r#type: QueryResultType,
}

#[derive(TS, Deserialize, Debug, Serialize, Clone)]
#[ts(export, export_to = "../../../src/types/")]
pub struct QueryResult {
    pub results: Vec<QueryResultItem>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../src/types/", rename_all = "PascalCase")]
#[serde(tag = "type")]
pub enum ResultPreview {
    File {
        path: String,
        filename: Option<String>,
        extension: Option<String>,
        #[serde(rename = "fileType")]
        file_type: FileType,
        size: String,
        #[serde(rename = "lastModified")]
        last_modified: Option<String>,
        content: String,
        #[serde(rename = "parsedContent")]
        parsed_content: Option<String>,
    },
    BrowserHistory {
        url: String,
        #[serde(rename = "imageUrl")]
        image_url: String,
        heading: String,
        subheading: String,
    },
    Script {
        path: String,
        #[serde(rename = "lastModified")]
        last_modified: String,
        language: String,
        content: String,
        #[serde(rename = "parsedContent")]
        parsed_content: Option<String>,
    },
    Calculator {
        #[serde(rename = "parsedContent")]
        parsed_content: String,
    },
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../src/types/")]
pub enum SFEvent {
    MainWindowShown,
    MainWindowHidden,
    MainWindowResized,
    SettingsWindowShown,
    SettingsWindowHidden,
    Query,
    QueryResult,
}

// an action takes the info it receives, does something with it, and returns nothing.
//   In other words: it acts on something and then exits, returning nothing back to the calling hook.
// a filter takes the info it receives, modifies it somehow, and returns it.
//   In other words: it filters something and passes it back to the hook for further use.

const MAIN_WINDOW_SHOWN: &'static str = "MainWindowShown";
const MAIN_WINDOW_HIDDEN: &'static str = "MainWindowHidden";
const SETTINGS_WINDOW_SHOWN: &'static str = "SettingsWindowShown";
const SETTINGS_WINDOW_HIDDEN: &'static str = "SettingsWindowHidden";
const RESIZED: &'static str = "MainWindowResized";
const QUERY: &'static str = "Query";
const QUERY_RESULT: &'static str = "QueryResult";

impl AsRef<str> for SFEvent {
    fn as_ref(&self) -> &str {
        match self {
            SFEvent::MainWindowShown => MAIN_WINDOW_SHOWN,
            SFEvent::MainWindowHidden => MAIN_WINDOW_HIDDEN,
            SFEvent::MainWindowResized => RESIZED,
            SFEvent::Query => QUERY,
            SFEvent::QueryResult => QUERY_RESULT,
            SFEvent::SettingsWindowShown => SETTINGS_WINDOW_SHOWN,
            SFEvent::SettingsWindowHidden => SETTINGS_WINDOW_HIDDEN,
        }
    }
}

impl FromStr for SFEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            MAIN_WINDOW_SHOWN => Ok(SFEvent::MainWindowShown),
            MAIN_WINDOW_HIDDEN => Ok(SFEvent::MainWindowHidden),
            SETTINGS_WINDOW_SHOWN => Ok(SFEvent::SettingsWindowShown),
            SETTINGS_WINDOW_HIDDEN => Ok(SFEvent::SettingsWindowHidden),
            RESIZED => Ok(SFEvent::MainWindowResized),
            QUERY => Ok(SFEvent::Query),
            QUERY_RESULT => Ok(SFEvent::QueryResult),
            _ => Err(()),
        }
    }
}

impl FileType {
    pub fn from_extension(file_extension: Option<&str>) -> FileType {
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
}

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
            FileType::CSV => "csv",
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

#[derive(TS, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[ts(export, export_to = "../../../src/types/")]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
                        FileType::from_extension(extension.as_deref())
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
