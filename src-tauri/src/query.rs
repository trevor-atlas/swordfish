use crate::datasource::DataSource;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum QueryMode {
    Search,
    Chat,
    Scripts,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Query {
    pub search_string: String,
    pub mode: QueryMode,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum QueryResultType {
    File,
    BrowserHistory,
    Script,
    Action,
    Calculator,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryResultItem {
    #[serde(rename = "iconPath")]
    pub icon_path: Option<String>,
    pub heading: String,
    pub subheading: String,
    pub value: String,
    pub preview: Option<Preview>,
    #[serde(rename = "type")]
    pub r#type: QueryResultType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryResult {
    pub results: Vec<QueryResultItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Preview {
    File {
        #[serde(rename = "iconPath")]
        icon_path: String,
        path: String,
        filename: String,
        extension: String,
        size: String,
        last_modified: String,
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
        #[serde(rename = "iconPath")]
        icon_path: String,
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
