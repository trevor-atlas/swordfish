use crate::{
    browser::browser::{collate_browser_history_data, query_collated_db, HistoryEntry},
    query::Query,
};
use std::{sync::mpsc, thread, time::SystemTime};

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
        thread::spawn(move || {
            let result = collate_browser_history_data(); // Function execution in new thread
            tx.send(result).unwrap();
        });
        self.last_updated = ts();

        rx.recv()
            .unwrap()
            .expect("Unable to update browser history cache");
        ()
    }

    fn query(&self, query: &Query) -> Option<Vec<HistoryEntry>> {
        match query_collated_db(query) {
            Ok(data) => Some(data),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}

/** get current timestamp */
fn ts() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
