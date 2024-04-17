use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub trait DataSource<T> {
    fn new() -> Self;
    fn update_cache();
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
        size: String,
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
#[ts(export, export_to = "../../../src/types/", rename_all = "lowercase")]
pub enum SFEvent {
    #[serde(rename = "mainwindow:shown")]
    MainWindowShown,
    #[serde(rename = "mainwindow:hidden")]
    MainWindowHidden,
    #[serde(rename = "mainwindow:resized")]
    MainWindowResized,
}

impl fmt::Display for SFEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SFEvent::MainWindowShown => write!(f, "mainwindow:shown"),
            SFEvent::MainWindowHidden => write!(f, "mainwindow:hidden"),
            SFEvent::MainWindowResized => write!(f, "mainwindow:resized"),
        }
    }
}

impl AsRef<str> for SFEvent {
    fn as_ref(&self) -> &str {
        match self {
            SFEvent::MainWindowShown => "mainwindow:shown",
            SFEvent::MainWindowHidden => "mainwindow:hidden",
            SFEvent::MainWindowResized => "mainwindow:resized",
        }
    }
}

impl FromStr for SFEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainwindow:shown" => Ok(SFEvent::MainWindowShown),
            "mainwindow:hidden" => Ok(SFEvent::MainWindowHidden),
            "mainwindow:resized" => Ok(SFEvent::MainWindowResized),
            _ => Err(()),
        }
    }
}
