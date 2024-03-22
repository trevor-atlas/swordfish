
use crate::datasource::{DataSource};


use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum QueryMode {
    Search,
    Chat,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Query {
    pub search_string: String,
    pub mode: QueryMode,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum QueryResultType {
    File,
    Clipboard,
    BrowserHistory,
    Script,
    Action,
    Calculator,
    Other,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryResultItem {
    pub heading: String,
    pub subheading: String,
    pub preview: Option<Preview>,
    #[serde(rename = "type")]
    pub r#type: QueryResultType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryResult {
    pub results: Vec<QueryResultItem>,
}

// What data do we need in order to render previews for different filetypes?
// macos preview is actually pretty good for a lot of these, so some aspects of that could be copied :)
// Requirements:
//   The data end of this should be easy to extend if new filetypes are discovered...
//   The client logic to render a preview should also be pretty simple to extend when new types are needed
//   Ideally we can rely on the file extension to identify how we handle things, but there may be a need for more complex detection :thinking:
//
// adobe files -> ??? no idea, need to research
// markdown -> can we render this in real-time, or should we preprocess it before serializing to the client? (what about mermaid and github extensions etc?)
// excel -> ???
// img -> path
// video -> path
// csv -> path
// xml -> probably similar to csv, not too complex
// json -> path
// code and misc text formats -> path
// svg -> path
// audio -> path
// pdf -> path is enough I think? I guess it depends on the system webview to some extent.
// fonts -> hmm not sure.
//   path is probably enough? I think the worst case is we load that font into the webview and display a hardcoded ABCDE... or pangram block like "Sphinx of Black Quartz, Judge My Vow"
//   it would also be nice to have the ability to type custom text in the preview
// binary -> as much metadata as possible
// zip, dmg, exe, rar and other compressed formats -> hard to say right now... this will require some research!
//   some can probably be preprocessed into a list of file info for the client to consume

pub enum FileContent {
    ParsedContent(String),
    Content(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Preview {
    File {
        path: String,
        filename: String,
        extension: String,
        size: String,
        last_modified: String,
        content: String,
        parsed_content: String,
    },
    Clipboard {
        filepath: Option<String>,
        content: String,
        parsed_content: String,
    },
    BrowserHistory {
        url: String,
        #[serde(rename = "imageUrl")]
        image_url: String,
        heading: String,
        subheading: String,
    },
}
