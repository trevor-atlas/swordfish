use fend_core;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum QueryMode {
    Search,
    Chat,
}

#[derive(Deserialize)]
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
    Other,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryResultEntry {
    pub heading: String,
    pub subheading: String,
    pub preview: Option<QueryResultPreview>,
    pub r#type: QueryResultType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryResult {
    inline_result: Option<String>,
    results: Vec<QueryResultEntry>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum QueryResultPreview {
    FilePreview {
        extension: String, // md, json, png, etc
        path: String,
        size_human: String,
        last_modified: String,
    },
    ClipboardPreview {
        filepath: Option<String>,
        text: Option<String>,
    },
    BrowserHistoryPreview {
        url: String,
        #[serde(rename = "imageUrl")]
        image_url: String,
        heading: String,
        subheading: String,
    },
}

#[tauri::command]
pub fn get_query_result(query: Query) -> QueryResult {
    let result_list = vec![
        QueryResultEntry {
            heading: "Exodia".to_string(),
            subheading: "The forbidden one".to_string(),
            preview: Some(QueryResultPreview::ClipboardPreview {
                filepath: None,
                text: Some("Its a clipboard entry :)".to_string()),
            }),
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "Halle Berry".to_string(),
            subheading: "Still hot tbh".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "The Pope (really)".to_string(),
            subheading: "He is old".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "MOOG".to_string(),
            subheading: "They kinda stink as a company but beep boop".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "HubSpot".to_string(),
            subheading: "Its okay! Really!".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "Stream Deck".to_string(),
            subheading: "It could be better, but it is aight".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "GGWP BGEZ".to_string(),
            subheading: "Nerds are so rude".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "Exodia".to_string(),
            subheading: "The forbidden one".to_string(),
            preview: Some(QueryResultPreview::ClipboardPreview {
                filepath: None,
                text: Some("Its a clipboard entry :)".to_string()),
            }),
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "Halle Berry".to_string(),
            subheading: "Still hot tbh".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "The Pope (really)".to_string(),
            subheading: "He is old".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "MOOG".to_string(),
            subheading: "They kinda stink as a company but beep boop".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "HubSpot".to_string(),
            subheading: "Its okay! Really!".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "Stream Deck".to_string(),
            subheading: "It could be better, but it is aight".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResultEntry {
            heading: "GGWP BGEZ".to_string(),
            subheading: "Nerds are so rude".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
    ]
    .iter()
    .cloned()
    .filter(|item| {
        item.heading
            .to_lowercase()
            .contains(&query.search_string.to_lowercase())
    })
    .collect::<Vec<QueryResultEntry>>();

    let mut context = fend_core::Context::new();

    QueryResult {
        inline_result: match fend_core::evaluate(&query.search_string, &mut context) {
            Ok(r) => Some(r.get_main_result().to_string()),
            Err(_) => None,
        },
        results: result_list,
    }
}
