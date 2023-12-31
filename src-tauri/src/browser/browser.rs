use dirs::home_dir;
use fend_core::json;
use glob::glob;
use rusqlite::params;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{env, error, io};
use std::{fs, thread};
use tauri::api::path::data_dir;
use thiserror::Error;
use url::Url;

use crate::query::Query;

use super::history::static_configs::{
    arc_path, brave_path, chrome_path, firefox_path, safari_path,
};

fn exec(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("failed to execute cmd");

    String::from_utf8(output.stdout).unwrap()
}

fn file_exists(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        // Check if the metadata represents a file (not a directory or other type)
        if metadata.is_file() {
            return true;
        }
    }
    false
}

fn delete_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             browser TEXT NOT NULL,
             url TEXT NOT NULL UNIQUE,
             title TEXT NOT NULL,
             visit_count INTEGER NOT NULL,
             last_visit_time INTEGER NOT NULL,
             frecency_score REAL NOT NULL
         )",
        (),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error creating table {}", e);
            return Err(e);
        }
    }
}

fn insert_history_entries(conn: &mut Connection, history_entries: Vec<HistoryEntry>) -> Result<()> {
    let tx = conn.transaction()?;
    {
        let mut statement = match tx.prepare(
        "INSERT INTO history (browser, url, title, visit_count, last_visit_time, frecency_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    ) {
        Ok(stmt) => {
            println!("statement prepared");
            stmt
        },
        Err(e) => {
            println!("unable to prepare statement {}", e);
            return Err(e);
        }
    };

        for entry in history_entries.iter() {
            match statement.execute(params![
                entry.browser.to_str(),
                entry.url,
                entry.title,
                entry.visit_count,
                entry.last_visit_time,
                entry.frecency_score
            ]) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error instering {}", e);
                }
            };
        }
    }

    tx.commit();
    Ok(())
}

// if a browser is running you cannot read the history sqlite directly
// because it's locked. You have to copy it somewhere else and use that copy instead
fn copy_browser_sqlite_to_tmpdir(from: &str, name: &str) {
    let mut dest_path = env::temp_dir();
    dest_path.push(format!("sf-history-{}.sqlite", name));
    match fs::copy(from, &dest_path) {
        Ok(_) => {
            println!("Copied to {:?}", dest_path)
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    };
}

fn prep_browser_sqlite_for_collation(configs: &Vec<HistorySchema>) {
    for config in configs {
        if config.path.contains("*") {
            for (i, entry) in glob(&config.path)
                .expect("Failed to read glob pattern")
                .enumerate()
            {
                match entry {
                    Ok(path) => {
                        copy_browser_sqlite_to_tmpdir(
                            &path.to_str().unwrap(),
                            &format!("{}-{}", &config.browser.to_str(), i + 1),
                        );
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        copy_browser_sqlite_to_tmpdir(&config.path, config.browser.to_str());
    }
}

fn get_copied_sqlite_paths_for_history_schema(schema: &HistorySchema) -> Vec<String> {
    let mut tmp_path = env::temp_dir();
    tmp_path.push(format!("sf-history-{}*.sqlite", schema.browser.to_str()));
    glob(&tmp_path.to_str().unwrap())
        .expect("error finding sqlite globs in tmp dir")
        .filter_map(|path| match path {
            Ok(p) => Some(p.to_string_lossy().to_string()),
            Err(_e) => None,
        })
        .collect::<Vec<String>>()
}

fn run_schema_query(path: &str, schema: &HistorySchema) -> Option<Vec<HistoryEntry>> {
    let connection = match Connection::open(path) {
        Ok(con) => con,
        Err(e) => {
            println!("Error connecting to db: {:?}", e);
            return None;
        }
    };

    let mut statement = match connection.prepare(&schema.query) {
        Ok(val) => val,
        Err(e) => {
            println!("error running query '{:?}' for {:?}", e, schema.browser);
            return None;
        }
    };

    let characters_to_remove = "/#";
    let rows = match statement.query_map([], |row| {
        let mut hist = HistoryEntry {
            browser: schema.browser.clone(),
            url: row.get_unwrap(1),
            title: row.get_unwrap(2),
            visit_count: row.get_unwrap(3),
            last_visit_time: row.get_unwrap(4),
            frecency_score: 0.0,
        };
        hist.frecency_score = calculate_frecency(&hist);

        match Url::parse(hist.url.as_str()) {
            Ok(mut url) => {
                url.set_query(None);
                let updated = url.as_str().trim_start_matches(characters_to_remove);
                hist.url = updated.to_string();
            }
            Err(e) => {}
        }
        Ok(hist)
    }) {
        Ok(rows) => rows.collect::<Result<Vec<_>, _>>(),
        Err(e) => {
            println!("error for {} data row {}", schema.browser.to_str(), e);
            return None;
        }
    };

    match rows {
        Ok(rows) => Some(
            rows.into_iter()
                .filter(|row| {
                    if row.url.contains("localhost")
                        || row.url.contains("127.0.0.1")
                        || row.url.contains("0.0.0.0")
                    {
                        false
                    } else {
                        true
                    }
                })
                .collect::<Vec<HistoryEntry>>(),
        ),
        Err(e) => {
            println!("error for {} data row {}", schema.browser.to_str(), e);
            None
        }
    }
}

#[derive(Error, Debug)]
pub enum BrowserHistoryCollationError {
    #[error("unable to copy history files to aggregate location")]
    UnableToCopyFiles,
    #[error("unable to establish a connection to combined history database")]
    UnableToConnectToCollatedDB,
    #[error("unable to query combined history database")]
    UnableToQueryCollatedDB,
    #[error("unable to map combined history database into runtime data (serialization error)")]
    UnableToSerializeCollatedDB,
    #[error("unable to locate user data dir, stopping collation")]
    UnableToLocateDataDir,
    #[error("unable to extract combined history data from mutex")]
    UnableToExtractCombinedHistoryMutex,
}

pub fn collate_browser_history_data() -> Result<(), BrowserHistoryCollationError> {
    let schemas: Vec<HistorySchema> = Browser::variants()
        .iter()
        .filter_map(|b| get_browser_history_schema(b))
        .collect();
    println!("found {} schemas", schemas.len());
    // prep_browser_sqlite_for_collation(&schemas);

    // Create an Arc and Mutex to safely share the 'history' Vec among threads
    let history = Arc::new(Mutex::new(Vec::<HistoryEntry>::new()));

    // Collect thread handles in a Vec
    let handles: Vec<_> = schemas
        .clone()
        .iter()
        .map(|schema| schema.clone())
        .map(|schema_clone| {
            // Clone the Arc for each thread
            let history_clone = history.clone();

            // Spawn a thread for each schema
            thread::spawn(move || {
                if schema_clone.path.contains("*") {
                    for (i, entry) in glob(&schema_clone.path)
                        .expect("Failed to read glob pattern")
                        .enumerate()
                    {
                        match entry {
                            Ok(path) => {
                                copy_browser_sqlite_to_tmpdir(
                                    &path.to_str().unwrap(),
                                    &format!("{}-{}", &schema_clone.browser.to_str(), i + 1),
                                );
                            }
                            Err(e) => println!("{:?}", e),
                        }
                    }
                } else {
                    copy_browser_sqlite_to_tmpdir(
                        &schema_clone.path,
                        schema_clone.browser.to_str(),
                    );
                }

                let entries = get_copied_sqlite_paths_for_history_schema(&schema_clone)
                    .iter()
                    .filter_map(|path| run_schema_query(path, &schema_clone))
                    .flatten()
                    .collect::<Vec<HistoryEntry>>();

                // Lock the Mutex to update the 'history' Vec
                let mut history = history_clone.lock().unwrap();
                history.extend(entries);
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Extract the final 'history' Vec
    let history = match Arc::try_unwrap(history) {
        Ok(mutex) => mutex.into_inner(),
        Err(e) => {
            println!("Error extracting combined browser history: {:?}", e);
            return Err(BrowserHistoryCollationError::UnableToExtractCombinedHistoryMutex);
        }
    }
    .unwrap_or(vec![]);
    println!("collated histories with length {}", history.len());

    if let Some(path) = get_collated_db_path() {
        let p = path.to_str().unwrap();
        if file_exists(p) {
            println!("deleting {}", p);
            let _ = delete_file(p);
        }
    }

    let mut connection = get_collated_db_connection()?;
    match create_table(&connection) {
        Ok(_) => match insert_history_entries(&mut connection, history) {
            Ok(_) => {
                println!("Wrote history to collated_browser_history db")
            }
            Err(e) => {
                println!("Error inserting into collated database {}", e);
                return Err(BrowserHistoryCollationError::UnableToQueryCollatedDB);
            }
        },
        Err(e) => {
            println!("Error creating collated database table {}", e);
            return Err(BrowserHistoryCollationError::UnableToQueryCollatedDB);
        }
    };
    Ok(())
}

pub fn get_collated_db_path() -> Option<PathBuf> {
    match data_dir() {
        Some(mut dir) => {
            dir.push("collated_browser_history.db");
            Some(dir)
        }
        None => None,
    }
}

pub fn get_collated_db_connection() -> Result<Connection, BrowserHistoryCollationError> {
    let collated_db_location = match get_collated_db_path() {
        Some(dir) => dir,
        None => {
            return Err(BrowserHistoryCollationError::UnableToLocateDataDir);
        }
    };

    let connection = match Connection::open(&collated_db_location) {
        Ok(con) => con,
        Err(e) => {
            println!(
                "Error connecting to \"{}\" db: {:?}",
                collated_db_location.display(),
                e
            );
            return Err(BrowserHistoryCollationError::UnableToConnectToCollatedDB);
        }
    };
    Ok(connection)
}

pub fn query_collated_db(query: &Query) -> Result<Vec<HistoryEntry>, BrowserHistoryCollationError> {
    let connection = get_collated_db_connection()?;

    let query_statement = "SELECT * FROM history WHERE title like '%' || ? || '%' OR url like '%' || ? || '%' ORDER BY frecency_score DESC LIMIT 10";

    let mut statement = match connection.prepare(query_statement) {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            return Err(BrowserHistoryCollationError::UnableToQueryCollatedDB);
        }
    };

    let entries = statement.query_map(params![query.search_string, query.search_string], |row| {
        Ok(HistoryEntry {
            browser: Browser::from_str(row.get::<_, String>("browser")?.as_str()), // Convert String to Browser enum
            url: row.get("url")?,
            title: row.get("title")?,
            visit_count: row.get("visit_count")?,
            last_visit_time: row.get("last_visit_time")?,
            frecency_score: row.get("frecency_score")?,
        })
    });

    match entries {
        Ok(values) => Ok(values.map(|result| result.unwrap()).collect()),
        Err(e) => {
            println!("Error reading from collate_browser_history_data {}", e);
            return Err(BrowserHistoryCollationError::UnableToSerializeCollatedDB);
        }
    }
}

fn calculate_frecency(history: &HistoryEntry) -> f64 {
    let current_time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    // Set the weights for the different factors that contribute to the frecency score
    let visit_weight = 0.80;
    let age_weight = 0.20;
    let age = if history.last_visit_time > current_time as i64 {
        0
    } else {
        (current_time as u128 - history.last_visit_time as u128) / (1 * 60 * 60 * 24)
    };

    if history.visit_count <= 0 {
        return 0.0;
    }

    // Calculate the frecency score using the weights and age
    let score = (history.visit_count as f64 * visit_weight) - (age as f64 * age_weight);
    if score <= 0.0 {
        return 0.0;
    }
    score
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum Browser {
    Arc,
    Chrome,
    Firefox,
    Safari,
    Brave,
    // Opera,
    // Vivaldi,
    // Chromium,
    // Edge,
}

impl Browser {
    pub fn variants() -> &'static [Browser] {
        &[
            Browser::Arc,
            Browser::Chrome,
            Browser::Firefox,
            Browser::Safari,
            Browser::Brave,
            // Opera,
            // Edge,
            // Vivaldi,
            // Chromium,
        ]
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Arc => "Arc",
            Self::Chrome => "Chrome",
            Self::Firefox => "Firefox",
            Self::Safari => "Safari",
            Self::Brave => "Brave",
            // Self::Opera => "Opera",
            // Self::Edge => "Edge",
            // Self::Vivaldi => "Vivaldi",
            // Self::Chromium => "Chromium",
        }
    }
    pub fn from_str(str: &str) -> Self {
        match str {
            "Arc" => Self::Arc,
            "Chrome" => Self::Chrome,
            "Firefox" => Self::Firefox,
            "Safari" => Self::Safari,
            "Brave" => Self::Brave,
            _ => panic!()
            // Self::Opera => "Opera",
            // Self::Edge => "Edge",
            // Self::Vivaldi => "Vivaldi",
            // Self::Chromium => "Chromium",
        }
    }
}

//* An entry from browser history */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryEntry {
    pub browser: Browser,
    pub url: String,
    pub title: String,
    pub visit_count: i64,
    /* timestamp in seconds */
    pub last_visit_time: i64,
    pub frecency_score: f64,
}

//* Definition of a single browser history sqlite location and how to get the data we need out */
#[derive(Debug, Clone)]
pub struct HistorySchema {
    pub browser: Browser,
    pub path: String,
    pub query: String,
}

pub fn get_browser_history_schema(browser: &Browser) -> Option<HistorySchema> {
    let default_query = "SELECT id, url, title, visit_count, CAST(((CAST(last_visit_time as REAL) - 11644473600000000) / 1000000) as BIGINT)as last_visit_time FROM urls ORDER BY visit_count DESC";

    let (maybe_path, query) = match browser {
        Browser::Arc => (arc_path(), default_query),
        Browser::Chrome => (chrome_path(), default_query),
        Browser::Firefox => (firefox_path(), "SELECT id, url, COALESCE(title, \"\"), visit_count, COALESCE(CAST(last_visit_date as INTEGER), 0) FROM moz_places ORDER BY visit_count DESC"),
        Browser::Safari => (safari_path(), "SELECT i.id, i.url, COALESCE(v.title, \"\"), i.visit_count, CAST(v.visit_time + 978307200 as BIGINT) as visit_time FROM history_items i LEFT JOIN history_visits v ON i.id = v.history_item ORDER BY i.visit_count DESC"),
        Browser::Brave => (brave_path(), default_query),
        // Opera => (opera_path(), DEFAULT_QUERY),
        // Edge => (edge_path(), DEFAULT_QUERY),
        // Vivaldi => (vivaldi_path(), DEFAULT_QUERY),
        // Chromium => (chromium_path(), DEFAULT_QUERY),
    };

    let home_path_buf = match home_dir() {
        Some(dir) => dir,
        None => panic!("could not get a valid home dir!"),
    };

    let home_path_str = match home_path_buf.as_path().to_str() {
        Some(str_path) => str_path,
        None => panic!("could not parse home dir to string!"),
    };

    match maybe_path {
        Some(p) => {
            let final_path = p.replace("{}", home_path_str);
            Some(HistorySchema {
                browser: *browser,
                path: final_path,
                query: query.to_string(),
            })
        }
        None => None,
    }
}
