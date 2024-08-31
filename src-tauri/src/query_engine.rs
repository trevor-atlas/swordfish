use std::{
    fs::{self, File},
    sync::mpsc,
    thread::spawn,
};

use fend_core::{FendResult, SpanKind};
use serde::Serialize;
use tauri::{api::file, utils::config::parse};

use crate::settings::AppConfig;
use swordfish_types::{
    DataSource, FileInfo, Query, QueryMode, QueryResult, QueryResultItem, QueryResultType,
    ResultPreview,
};

use crate::{
    datasource::BrowserHistoryDataSource,
    search::filename::{search, FileDataSource},
    utilities::{cache_all_app_icons, get_cached_app_icon_path, get_favicon_path},
};

pub trait QueryInterface {
    fn new() -> Self;
    fn query(&self, query: Query) -> QueryResult;
}

pub struct QueryEngine {
    browser_history: BrowserHistoryDataSource,
    file_data: FileDataSource,
}

fn is_empty_query(query: &Query) -> bool {
    query.search_string.is_empty() || query.search_string.trim().is_empty()
}

fn search_files(results: Vec<FileInfo>) -> Vec<QueryResultItem> {
    results
        .iter()
        .map(|item| QueryResultItem {
            heading: item.file_name.clone().unwrap_or("unnamed file".to_string()),
            subheading: item.path.clone(),
            value: item.path.clone(),
            icon_path: (item.extension == Some("app".to_string()))
                .then(|| {
                    item.file_name
                        .as_ref()
                        .and_then(|name| get_cached_app_icon_path(name))
                })
                .flatten(),
            preview: Some(ResultPreview::File {
                path: item.path.clone(),
                filename: item.file_name.clone(),
                extension: item.extension.clone(),
                file_type: item.file_type.clone(),
                size: item.size.to_string(),
                last_modified: item.last_modified.and_then(|i| Some(i.to_string())),
                content: "".to_string(),
                parsed_content: Some("".to_string()),
            }),
            r#type: QueryResultType::File,
        })
        .collect()
}

fn search_browser_history(q: &Query, tx: mpsc::Sender<QueryResultItem>) {
    let hist = BrowserHistoryDataSource::new();
    if let Some(hist_items) = hist.query(q) {
        for item in hist_items.iter() {
            tx.send(QueryResultItem {
                heading: item.title.clone(),
                subheading: item.url.clone(),
                value: item.url.clone(),
                preview: Some(ResultPreview::BrowserHistory {
                    url: item.url.clone(),
                    image_url: "".to_string(),
                    heading: item.title.clone(),
                    subheading: item.url.clone(),
                }),
                icon_path: get_favicon_path(item.url.as_str()),
                r#type: QueryResultType::BrowserHistory,
            })
            .unwrap_or_else(|e| {
                println!("Error sending history item: {}", e);
            });
        }
    }
}

impl QueryInterface for QueryEngine {
    fn new() -> Self {
        let browser_history = BrowserHistoryDataSource::new();
        let file_data = FileDataSource::new();
        Self {
            browser_history,
            file_data,
        }
    }

    fn query(&self, query: Query) -> QueryResult {
        let mut results: Vec<QueryResultItem> = vec![];
        if let Some(calculator_result) = get_calculator_result(&query) {
            if calculator_result.heading != query.search_string {
                results.push(calculator_result);
            }
        }
        match query.mode {
            QueryMode::Search => {
                if is_empty_query(&query) {
                    return QueryResult { results: vec![] };
                }

                if let Some(dirs) = self.file_data.query(&query) {
                    QueryResult {
                        results: search_files(dirs),
                    }
                } else {
                    QueryResult { results: vec![] }
                }
            }
            QueryMode::BrowserHistory => {
                if is_empty_query(&query) {
                    return QueryResult { results: vec![] };
                }
                let (tx, rx) = mpsc::channel();
                let q = query.clone();

                let history_handle = spawn(move || {
                    search_browser_history(&q, tx);
                });

                history_handle.join().unwrap();
                while let Ok(i) = rx.recv() {
                    results.push(i.clone());
                }
                QueryResult { results }
            }
            QueryMode::Chat => QueryResult { results: vec![] },
            QueryMode::Scripts => {
                let file_content =
                    match fs::read_to_string("/Users/atlas/Desktop/swordfish-test-script.ts") {
                        Ok(content) => content,
                        Err(_) => "".to_string(),
                    };

                return QueryResult {
                    results: vec![QueryResultItem {
                        heading: "Scripts".to_string(),
                        subheading: "Run scripts".to_string(),
                        value: "Scripts".to_string(),
                        icon_path: None,
                        r#type: QueryResultType::Script,
                        preview: Some(ResultPreview::Script {
                            path: "/Desktop".to_string(),
                            last_modified: "2024-08-24".to_string(),
                            language: "ts".to_string(),
                            content: "".to_string(),
                            parsed_content: Some(file_content),
                        }),
                    }],
                };
            }
        }
    }
}

fn get_calculator_result(query: &Query) -> Option<QueryResultItem> {
    let mut context = fend_core::Context::new();
    if let Ok(fend_result) = fend_core::evaluate(&query.search_string, &mut context) {
        if fend_result.get_main_result().is_empty() {
            return None;
        }

        let parsed_result = format_calculator_result(&fend_result);

        return Some(QueryResultItem {
            heading: fend_result.get_main_result().to_string(),
            subheading: parsed_result,
            value: fend_result.get_main_result().to_string(),
            icon_path: None,
            r#type: QueryResultType::Calculator,
            preview: None,
        });
    }
    None
}

fn format_calculator_result(fend_result: &FendResult) -> String {
    let calculator_content = fend_result
        .get_main_result_spans()
        .into_iter()
        .filter(|span| !span.string().is_empty())
        .map(|span| match span.kind() {
            SpanKind::Boolean => format!(
                "<span class=\"calculator-boolean\">{}</span>",
                span.string()
            ),
            SpanKind::Number => {
                format!("<span class=\"calculator-number\">{}</span>", span.string())
            }
            SpanKind::BuiltInFunction => format!(
                "<span class=\"calculator-builtin-fn\">{}</span>",
                span.string()
            ),
            SpanKind::Keyword => format!(
                "<span class=\"calculator-keyword\">{}</span>",
                span.string()
            ),
            SpanKind::String => {
                format!("<span class=\"calculator-string\">{}</span>", span.string())
            }
            SpanKind::Date => format!("<span class=\"calculator-date\">{}</span>", span.string()),
            SpanKind::Whitespace => format!(
                "<span class=\"calculator-whitespace\">{}</span>",
                span.string()
            ),
            SpanKind::Ident => format!("<span class=\"calculator-ident\">{}</span>", span.string()),
            SpanKind::Other => format!("<span class=\"calculator-other\">{}</span>", span.string()),
            _ => format!(
                "<span class=\"calculator-unknown\">{}</span>",
                span.string()
            ),
        })
        .collect::<Vec<_>>()
        .join("");
    format!("<span class=\"calculator\">{}</span>", calculator_content)
}
