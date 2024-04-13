use std::{
    fs::File,
    io::{BufReader, BufWriter},
    sync::mpsc,
    thread::spawn,
};

use crate::{
    datasource::{BrowserHistoryDataSource, DataSource},
    query::{Preview, Query, QueryMode, QueryResult, QueryResultItem, QueryResultType},
    search::filename::search,
    utilities::{
        get_app_icon_cache_path, get_cache_path, get_cached_app_icon_path, get_favicon_path,
    },
};

pub trait QueryInterface {
    fn new() -> Self;
    fn query(&self, query: Query) -> QueryResult;
}

use icns::{IconFamily, IconType};
use plist::Value;
use std::path::PathBuf;

pub fn cache_app_icon_path(app_bundle_path: &str, app_name: &str) -> Option<PathBuf> {
    if let Some(icon_path) = get_cached_app_icon_path(&app_name) {
        return Some(PathBuf::from(icon_path));
    }

    let mut app_bundle_path = PathBuf::from(app_bundle_path);
    app_bundle_path.push("Contents");
    let mut plist_path = app_bundle_path.clone();
    plist_path.push("Info.plist");

    let info_plist = Value::from_file(plist_path).ok()?;

    let icon_file_name = info_plist
        .as_dictionary()?
        .get("CFBundleIconFile")
        .and_then(Value::as_string)?;

    // macOS does not require the extension for .icns files in the Info.plist.
    // Ensure it has the .icns extension.
    let icon_file_name = if icon_file_name.ends_with(".icns") {
        icon_file_name.to_string()
    } else {
        format!("{}.icns", icon_file_name)
    };

    // Construct the path to the icon file within the .app bundle
    let icon_path = app_bundle_path
        .join("Resources")
        .join(icon_file_name.clone());

    get_app_icon_cache_path()
        .and_then(|mut dir| {
            dir.push(format!("{}.png", app_name));
            Some(dir)
        })
        .map(|cache_path| {
            File::open(icon_path.clone())
                .ok()
                .and_then(|f| Some(BufReader::new(f)))
                .and_then(|file| {
                    let icon_family = IconFamily::read(file).ok()?;
                    let file = BufWriter::new(File::create(cache_path.clone()).unwrap());
                    icon_family
                        .get_icon_with_type(IconType::RGB24_128x128)
                        .or(icon_family.get_icon_with_type(IconType::RGBA32_512x512_2x))
                        .or(icon_family.get_icon_with_type(IconType::RGBA32_512x512))
                        .or(icon_family.get_icon_with_type(IconType::RGBA32_256x256_2x))
                        .or(icon_family.get_icon_with_type(IconType::RGBA32_256x256))
                        .or(icon_family.get_icon_with_type(IconType::RGBA32_128x128_2x))
                        .or(icon_family.get_icon_with_type(IconType::RGBA32_128x128))
                        .ok()
                        .and_then(|i| i.write_png(file).ok());
                    Some(cache_path)
                })
        })
        .flatten()
}

pub struct QueryEngine {}

impl QueryInterface for QueryEngine {
    fn new() -> Self {
        BrowserHistoryDataSource::new().update_cache();
        Self {}
    }

    fn query(&self, query: Query) -> QueryResult {
        if query.search_string.is_empty() || query.search_string.trim().is_empty() {
            return QueryResult { results: vec![] };
        }
        let mut results: Vec<QueryResultItem> = vec![];
        if let Some(calculator_result) = get_calculator_result(&query) {
            results.push(calculator_result);
        }
        match query.mode {
            QueryMode::Search => {
                let (tx, rx) = mpsc::channel();
                let q = query.clone();
                let files_handle = spawn(move || {
                    search(&q).and_then(|files| {
                        for item in files.iter() {
                            let heading = item.file_name.clone();
                            let res = QueryResultItem {
                                heading: heading.unwrap_or("unnamed file".to_string()),
                                subheading: item.path.clone(),
                                value: item.path.clone(),
                                icon_path: if item.extension == Some("app".to_string())
                                    && item.file_name.is_some()
                                {
                                    cache_app_icon_path(
                                        item.path.as_str(),
                                        item.file_name.clone()?.as_str(),
                                    )
                                    .map(|p| p.to_string_lossy().to_string())
                                } else {
                                    None
                                },
                                preview: Some(Preview::File {
                                    path: item.path.clone(),
                                    filename: item.file_name.clone(),
                                    extension: item.extension.clone(),
                                    size: item.size.to_string(),
                                    last_modified: item
                                        .last_modified
                                        .and_then(|i| Some(i.to_string())),
                                    content: "".to_string(),
                                    parsed_content: Some("".to_string()),
                                }),
                                r#type: QueryResultType::File,
                            };
                            tx.send(res).unwrap_or_else(|e| {
                                println!("Error sending history item: {}", e);
                            });
                        }
                        Some(files)
                    })
                });

                files_handle.join().unwrap();

                while let Ok(i) = rx.recv() {
                    results.push(i.clone());
                }

                QueryResult { results: results }
            }
            QueryMode::BrowserHistory => {
                let (tx, rx) = mpsc::channel();
                let q = query.clone();
                let history_handle = spawn(move || {
                    let hist = BrowserHistoryDataSource::new();
                    if let Some(hist_items) = hist.query(&q) {
                        for item in hist_items.iter() {
                            tx.send(QueryResultItem {
                                heading: item.title.clone(),
                                subheading: item.url.clone(),
                                value: item.url.clone(),
                                preview: None,
                                icon_path: get_favicon_path(item.url.as_str()),
                                r#type: QueryResultType::BrowserHistory,
                            })
                            .unwrap_or_else(|e| {
                                println!("Error sending history item: {}", e);
                            });
                        }
                    }
                });

                history_handle.join().unwrap();
                while let Ok(i) = rx.recv() {
                    results.push(i.clone());
                }
                QueryResult { results: results }
            }
            QueryMode::Chat => QueryResult { results: vec![] },
            QueryMode::Scripts => QueryResult { results: vec![] },
        }
    }
}

fn get_calculator_result(query: &Query) -> Option<QueryResultItem> {
    let mut context = fend_core::Context::new();
    if let Ok(fend_result) = fend_core::evaluate(&query.search_string, &mut context) {
        if fend_result.get_main_result().is_empty() {
            return None;
        }
        let preview = Preview::Calculator {
            parsed_content: format!(
                "<span class=\"calculator\">{}</span>",
                fend_result
                    .get_main_result_spans()
                    .into_iter()
                    .filter(|span| !span.string().is_empty())
                    .map(|span| match span.kind() {
                        fend_core::SpanKind::Boolean =>
                            format!("<span class='calculator-boolean'>{}</span>", span.string()),
                        fend_core::SpanKind::Number =>
                            format!("<span class='calculator-number'>{}</span>", span.string()),
                        fend_core::SpanKind::BuiltInFunction => format!(
                            "<span class='calculator-builtin-fn'>{}</span>",
                            span.string()
                        ),
                        fend_core::SpanKind::Keyword =>
                            format!("<span class='calculator-keyword'>{}</span>", span.string()),
                        fend_core::SpanKind::String =>
                            format!("<span class='calculator-string'>{}</span>", span.string()),
                        fend_core::SpanKind::Date =>
                            format!("<span class='calculator-date'>{}</span>", span.string()),
                        fend_core::SpanKind::Whitespace => format!(
                            "<span class='calculator-whitespace'>{}</span>",
                            span.string()
                        ),
                        fend_core::SpanKind::Ident =>
                            format!("<span class='calculator-ident'>{}</span>", span.string()),
                        fend_core::SpanKind::Other =>
                            format!("<span class='calculator-other'>{}</span>", span.string()),
                        _ => format!("<span class='calculator-unknown'>{}</span>", span.string()),
                    })
                    .collect::<Vec<_>>()
                    .join("")
            ),
        };
        return Some(QueryResultItem {
            heading: fend_result.get_main_result().to_string(),
            subheading: "".to_string(),
            value: fend_result.get_main_result().to_string(),
            icon_path: None,
            r#type: QueryResultType::Calculator,
            preview: Some(preview),
        });
    }
    None
}
