use std::fs::File;

use crate::{
    datasource::{BrowserHistoryDataSource, DataSource},
    query::{Preview, Query, QueryMode, QueryResult, QueryResultItem, QueryResultType},
    search::filename::search,
};

pub trait QueryInterface {
    fn new() -> Self;
    fn query(&self, query: Query) -> QueryResult;
}

pub struct QueryEngine {
    pub browser_history_datasource: BrowserHistoryDataSource,
}

impl QueryInterface for QueryEngine {
    fn new() -> Self {
        let mut browser_history_datasource = BrowserHistoryDataSource::new();
        browser_history_datasource.update_cache();
        Self {
            browser_history_datasource: browser_history_datasource,
        }
    }

    fn query(&self, query: Query) -> QueryResult {
        match query.mode {
            QueryMode::Search => {
                let mut results = vec![];

                let mut context = fend_core::Context::new();
                match fend_core::evaluate(&query.search_string, &mut context) {
                    Ok(r) => {
                        if !r.get_main_result().is_empty() {
                            results.push(QueryResultItem {
                                heading: r.get_main_result().to_string(),
                                subheading: "".to_string(),
                                preview: None,
                                r#type: QueryResultType::Calculator,
                            });
                        }
                    }
                    Err(_) => {}
                };

                // if let Some(hist_items) = self.browser_history_datasource.query(&query) {
                //     for item in hist_items {
                //         results.push(QueryResultItem {
                //             heading: item.title,
                //             subheading: item.url,
                //             preview: None,
                //             r#type: QueryResultType::BrowserHistory,
                //         });
                //     }
                // }

                if let Some(files) = search(&query) {
                    for item in files {
                        results.push(QueryResultItem {
                            heading: item.file_name.clone(),
                            subheading: item.path.clone(),
                            preview: Some(Preview::File {
                                path: item.path.clone(),
                                filename: item.file_name.clone(),
                                extension: item.extension,
                                size: item.size.to_string(),
                                last_modified: item.last_modified.to_string(),
                                content: "".to_string(),
                                parsed_content: "".to_string(),
                            }),
                            r#type: QueryResultType::File,
                        });
                    }
                }

                QueryResult { results: results }
            }
            QueryMode::Chat => QueryResult { results: vec![] },
        }
    }
}
