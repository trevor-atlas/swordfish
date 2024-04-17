use crate::{
    browser::browser::{collate_browser_history_data, query_collated_db, HistoryEntry},
    query::Query,
};
use std::{sync::mpsc, time::SystemTime};

pub struct BrowserHistoryDataSource {
    pub last_updated: u64,
}
pub trait DataSource<T> {
    fn new() -> Self;
    fn update_cache(&mut self);
    fn query(&self, query: &Query) -> Option<T>; // Assuming a simple String return type for the example
}

impl DataSource<Vec<HistoryEntry>> for BrowserHistoryDataSource {
    fn new() -> Self {
        Self { last_updated: 0 }
    }

    fn update_cache(&mut self) {
        let (tx, rx) = mpsc::channel();
        tokio::spawn(async move {
            let result = collate_browser_history_data();
            let _ = tx.send(result).map_err(|e| {
                println!("Error updating browser history cache: {}", e);
            });
        });
        self.last_updated = ts();

        let _ = rx.recv().map_err(|e| {
            println!("Unknown error while combining browser histories: {}", e);
        });
    }

    fn query(&self, query: &Query) -> Option<Vec<HistoryEntry>> {
        query_collated_db(query)
            .map_err(|e| {
                println!("Unknown error searching browser history {}", e);
            })
            .ok()
    }
}

/** get current timestamp */
fn ts() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
