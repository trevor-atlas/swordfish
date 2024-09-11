use crate::windows::hide_main_window;
use crate::{
    browser_data_source::{BrowserHistoryDataSource, HistoryEntry},
    file_data_source::FileDataSource,
    windows::acquire_main_window,
};
use axum::error_handling::HandleErrorLayer;
use axum::BoxError;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use fend_core::{FendResult, SpanKind};
use reqwest::StatusCode;
use serde_variant::to_variant_name;
use std::fs::{self};
use std::time::Duration;
use swordfish_types::{
    DataSource, Query, QueryMode, QueryResult, ReceivedEvent, ResultDetails, ResultItem,
    ResultType, SFEvent,
};
use swordfish_utilities::get_favicon_path;
use tauri::AppHandle;
use tauri::Manager;
use tower::ServiceBuilder;

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

impl QueryEngine {
    pub fn start_ipc_server(handle: &AppHandle) {
        let handle = handle.clone();
        tokio::spawn(async move {
            let ipc_server = Router::new()
                .route("/emit", post(QueryEngine::handle_emit))
                .with_state(handle)
                .layer(
                    ServiceBuilder::new()
                        .layer(HandleErrorLayer::new(|error: BoxError| async move {
                            if error.is::<tower::timeout::error::Elapsed>() {
                                Ok(StatusCode::REQUEST_TIMEOUT)
                            } else {
                                Err((
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    format!("Unhandled internal error: {error}"),
                                ))
                            }
                        }))
                        .timeout(Duration::from_secs(5))
                        .into_inner(),
                );
            let listener = tokio::net::TcpListener::bind("0.0.0.0:2357").await.unwrap();
            axum::serve(listener, ipc_server).await.unwrap();
        });
    }

    async fn handle_emit(
        State(state): State<AppHandle>,
        Json(input): Json<ReceivedEvent>,
    ) -> Result<impl IntoResponse, StatusCode> {
        println!("input is {:?}", input);
        let w = acquire_main_window(&state);
        match input {
            ReceivedEvent::CloseWindow { window_ident } => {
                println!("Event to close window '{:?}'", window_ident);
                if let Ok(bool) = w.is_visible() {
                    if bool {
                        state
                            .emit_all(to_variant_name(&SFEvent::MainWindowHidden).unwrap(), ())
                            .ok();
                        hide_main_window(state.clone());
                    }
                }
            }
            ReceivedEvent::Query { query } => {
                println!("Event to query with '{}'", query.search_string)
            }
            ReceivedEvent::OpenWindow { window_ident } => {
                println!("Event to open window '{:?}'", window_ident)
            }
            ReceivedEvent::RunScript { script_name } => {
                println!("Event to run script'{}'", script_name)
            }
        }

        Ok(())
    }
}

impl QueryInterface for QueryEngine {
    fn new() -> Self {
        let mut browser_history = BrowserHistoryDataSource::new("history");
        let mut file_data = FileDataSource::new("sf_cache");
        browser_history.update_cache();
        file_data.update_cache();

        Self {
            browser_history,
            file_data,
        }
    }

    fn query(&self, query: Query) -> QueryResult {
        let mut results: Vec<ResultItem> = vec![];
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
                        results: dirs
                            .iter()
                            .map(|info| ResultItem::from(info.to_owned()))
                            .collect(),
                    }
                } else {
                    QueryResult { results: vec![] }
                }
            }
            QueryMode::BrowserHistory => {
                if is_empty_query(&query) {
                    return QueryResult { results: vec![] };
                }
                self.browser_history
                    .query(&query)
                    .map(|entries| QueryResult {
                        results: entries
                            .iter()
                            .map(|item| ResultItem {
                                heading: item.title.clone(),
                                subheading: item.url.clone(),
                                value: item.url.clone(),
                                details: Some(ResultDetails::BrowserHistory {
                                    url: item.url.clone(),
                                    image_url: "".to_string(),
                                    heading: item.title.clone(),
                                    subheading: item.url.clone(),
                                }),
                                icon_path: get_favicon_path(item.url.as_str()),
                                r#type: ResultType::BrowserHistory,
                            })
                            .collect(),
                    })
                    .unwrap_or(QueryResult { results: vec![] })
            }
            QueryMode::Chat => QueryResult { results: vec![] },
            QueryMode::Scripts => {
                let file_content =
                    match fs::read_to_string("/Users/atlas/Desktop/swordfish-test-script.ts") {
                        Ok(content) => content,
                        Err(_) => "".to_string(),
                    };

                QueryResult {
                    results: vec![ResultItem {
                        heading: "Scripts".to_string(),
                        subheading: "Run scripts".to_string(),
                        value: "Scripts".to_string(),
                        icon_path: None,
                        r#type: ResultType::Script,
                        details: Some(ResultDetails::Script {
                            path: "/Desktop".to_string(),
                            last_modified: "2024-08-24".to_string(),
                            language: "ts".to_string(),
                            content: "".to_string(),
                            parsed_content: Some(file_content),
                        }),
                    }],
                }
            }
        }
    }
}

fn get_calculator_result(query: &Query) -> Option<ResultItem> {
    let mut context = fend_core::Context::new();
    if let Ok(fend_result) = fend_core::evaluate(&query.search_string, &mut context) {
        if fend_result.get_main_result().is_empty() {
            return None;
        }

        let parsed_result = format_calculator_result(&fend_result);

        return Some(ResultItem {
            heading: fend_result.get_main_result().to_string(),
            subheading: parsed_result,
            value: fend_result.get_main_result().to_string(),
            icon_path: None,
            r#type: ResultType::Calculator,
            details: None,
        });
    }
    None
}

fn format_calculator_result(fend_result: &FendResult) -> String {
    let calculator_content = fend_result
        .get_main_result_spans()
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
