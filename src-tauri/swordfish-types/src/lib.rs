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
#[ts(export, export_to = "../../../src/types/")]
pub enum SFEvent {
    MainWindowShown,
    MainWindowHidden,
    MainWindowResized,
    Query,
    QueryResult,
}

// an action takes the info it receives, does something with it, and returns nothing.
//   In other words: it acts on something and then exits, returning nothing back to the calling hook.
// a filter takes the info it receives, modifies it somehow, and returns it.
//   In other words: it filters something and passes it back to the hook for further use.

const SHOWN: &'static str = "MainWindowShown";
const HIDDEN: &'static str = "MainWindowHidden";
const RESIZED: &'static str = "MainWindowResized";
const QUERY: &'static str = "Query";
const QUERY_RESULT: &'static str = "QueryResult";

impl AsRef<str> for SFEvent {
    fn as_ref(&self) -> &str {
        match self {
            SFEvent::MainWindowShown => SHOWN,
            SFEvent::MainWindowHidden => HIDDEN,
            SFEvent::MainWindowResized => RESIZED,
            SFEvent::Query => QUERY,
            SFEvent::QueryResult => QUERY_RESULT,
        }
    }
}

impl FromStr for SFEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            SHOWN => Ok(SFEvent::MainWindowShown),
            HIDDEN => Ok(SFEvent::MainWindowHidden),
            RESIZED => Ok(SFEvent::MainWindowResized),
            QUERY => Ok(SFEvent::Query),
            QUERY_RESULT => Ok(SFEvent::QueryResult),
            _ => Err(()),
        }
    }
}
