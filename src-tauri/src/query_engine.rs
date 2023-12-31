use crate::{
    query::{Query, QueryMode, QueryResult, QueryResultItem, QueryResultType},
    DataSource::{BrowserHistoryDataSource, DataSource},
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
                let history = self.browser_history_datasource.query(&query);
                let mut results = vec![];

                match history {
                    Some(items) => {
                        for item in items {
                            results.push(QueryResultItem {
                                heading: item.title,
                                subheading: item.url,
                                preview: None,
                                r#type: QueryResultType::BrowserHistory,
                            });
                        }
                    }
                    None => (),
                };

                let filtered_results = results
                    .iter()
                    .cloned()
                    .filter(|item| {
                        item.heading
                            .to_lowercase()
                            .contains(&query.search_string.to_lowercase())
                    })
                    .collect::<Vec<QueryResultItem>>();

                let mut context = fend_core::Context::new();

                QueryResult {
                    inline_result: match fend_core::evaluate(&query.search_string, &mut context) {
                        Ok(r) => Some(r.get_main_result().to_string()),
                        Err(_) => None,
                    },
                    results: filtered_results,
                }
            }
            QueryMode::Chat => QueryResult {
                inline_result: None,
                results: vec![],
            },
        }
    }
}
