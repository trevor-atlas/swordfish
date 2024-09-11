use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs, path::Path, str::FromStr};
use swordfish_utilities::get_cached_app_icon_path;
use ts_rs::TS;

mod file_type;
use file_type::FileType;

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(
    export,
    export_to = "../../../src/types/",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum WindowIdent {
    Main,
    Settings,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../src/types/")]
#[serde(tag = "type")]
pub enum ReceivedEvent {
    // #[serde(rename_all = "camelCase")]
    Query { query: Query },
    // #[serde(rename_all = "camelCase")]
    OpenWindow { window_ident: WindowIdent },
    // #[serde(rename_all = "camelCase")]
    CloseWindow { window_ident: WindowIdent },
    // #[serde(rename_all = "camelCase")]
    RunScript { script_name: String },
}

pub trait DataSource<T> {
    fn new(name: &str) -> Self;
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
pub enum ResultType {
    File,
    BrowserHistory,
    Script,
    Action,
    Calculator,
}

#[derive(TS, Deserialize, Debug, Serialize, Clone)]
#[ts(export, export_to = "../../../src/types/")]
pub struct ResultItem {
    pub heading: String,
    pub subheading: String,
    pub value: String,
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    #[serde(rename = "type")]
    pub r#type: ResultType,
    pub details: Option<ResultDetails>,
}

#[derive(TS, Deserialize, Debug, Serialize, Clone)]
#[ts(export, export_to = "../../../src/types/")]
pub struct QueryResult {
    pub results: Vec<ResultItem>,
}

impl ResultItem {
    pub fn from(file_info: FileInfo) -> Self {
        Self {
            heading: file_info
                .file_name
                .clone()
                .unwrap_or("unnamed file".to_string()),
            subheading: file_info.path.clone(),
            value: file_info.path.clone(),
            icon_path: (file_info.extension == Some("app".to_string()))
                .then(|| {
                    file_info
                        .file_name
                        .as_ref()
                        .and_then(|name| get_cached_app_icon_path(name))
                })
                .flatten(),
            r#type: ResultType::File,
            details: Some(ResultDetails::File {
                path: file_info.path.clone(),
                filename: file_info.file_name,
                extension: file_info.extension,
                file_type: file_info.file_type,
                size: file_info.size.to_string(),
                last_modified: file_info.last_modified.map(|i| i.to_string()),
                content: "".to_string(),
                parsed_content: Some("".to_string()),
            }),
        }
    }
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../src/types/", rename_all = "PascalCase")]
#[serde(tag = "type")]
pub enum ResultDetails {
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
    RunScript,
    ScriptResult,
}

impl FromStr for FileType {
    type Err = ();

    fn from_str(file_path: &str) -> Result<FileType, ()> {
        let path = Path::new(file_path);
        let fname = path
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .to_lowercase();

        match fname.as_str() {
            "cargo.lock" => return Ok(FileType::Toml),
            "makefile" => return Ok(FileType::MakeFile),
            "brewfile" => return Ok(FileType::BrewFile),
            _ => {}
        }

        let ext = path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .to_lowercase();

        Ok(FileType::from_extension(Some(ext.as_str())))
    }
}

#[derive(TS, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[ts(export, export_to = "../../../src/types/")]
pub struct FileInfo {
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    pub path: String,
    pub extension: Option<String>,
    #[serde(rename = "fileType")]
    pub file_type: FileType,
    pub size: u64,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<u64>,
    pub created: Option<u64>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePathError;

impl FromStr for FileInfo {
    type Err = ParsePathError;

    fn from_str(filepath: &str) -> Result<Self, Self::Err> {
        let path = Path::new(filepath);
        let file_name = path.file_stem().and_then(OsStr::to_str).map(str::to_string);
        let extension = path.extension().and_then(OsStr::to_str).map(str::to_string);
        if let Ok(metadata) = fs::metadata(path) {
            Ok(Self {
                file_name,
                path: filepath.to_string(),
                extension: extension.clone(),
                file_type: if extension.clone().is_some() {
                    FileType::from_extension(extension.as_deref())
                } else if metadata.is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                },
                size: metadata.len(),
                last_modified: metadata
                    .modified()
                    .ok()
                    .map(|date| date.elapsed().ok())
                    .and_then(|date| date.map(|d| d.as_secs())),
                created: metadata
                    .created()
                    .ok()
                    .map(|date| date.elapsed().ok())
                    .and_then(|date| date.map(|d| d.as_secs())),
            })
        } else {
            Err(ParsePathError)
        }
    }
}

impl FileInfo {
    pub fn from_string(filepath: String) -> Option<Self> {
        match Self::from_str(&filepath) {
            Ok(file_info) => Some(file_info),
            Err(_) => None,
        }
    }
}
