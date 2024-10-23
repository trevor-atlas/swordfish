use crate::sqlite::SQLite;
use chrono::Local;
use fuzzy_matcher::skim::SkimMatcherV2;
use glob::glob;
use rayon::prelude::*;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use std::{cmp, env, fs};
use swordfish_types::{DataSource, Query};
use swordfish_utilities::get_favicon_cache_path;
use thiserror::Error;
use tokio::spawn;
use url::Url;

fn calculate_frecency(history: &HistoryEntry) -> f64 {
    let current_time = Local::now();
    let timestamp = current_time.timestamp();
    let visit_weight = 0.40;
    let age_weight = 0.60;
    if history.last_visit_time > timestamp {
        println!("Invalid timestamp {:?}", &history);
        return 0.0;
    }
    let age = (timestamp - history.last_visit_time) / (60 * 60 * 24);

    let score = (age as f64 * age_weight) + (history.visit_count as f64 * visit_weight);

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
    Opera,
    Vivaldi,
    Chromium,
    Edge,
}

impl Browser {
    pub fn variants() -> &'static [Browser] {
        &[
            Browser::Arc,
            Browser::Chrome,
            Browser::Firefox,
            Browser::Safari,
            Browser::Brave,
            Browser::Opera,
            Browser::Edge,
            Browser::Vivaldi,
            Browser::Chromium,
        ]
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Arc => "Arc",
            Self::Chrome => "Chrome",
            Self::Firefox => "Firefox",
            Self::Safari => "Safari",
            Self::Brave => "Brave",
            Self::Opera => "Opera",
            Self::Edge => "Edge",
            Self::Vivaldi => "Vivaldi",
            Self::Chromium => "Chromium",
        }
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "Arc" => Self::Arc,
            "Chrome" => Self::Chrome,
            "Firefox" => Self::Firefox,
            "Safari" => Self::Safari,
            "Brave" => Self::Brave,
            "Opera" => Self::Opera,
            "Edge" => Self::Edge,
            "Vivaldi" => Self::Vivaldi,
            "Chromium" => Self::Chromium,
            _ => panic!("invalid browser variant in from_str"),
        }
    }

    pub fn from_string(str: String) -> Self {
        Browser::from_str(str.as_str())
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
struct BrowserPaths {
    arc: Option<PathBuf>,
    chrome: Option<PathBuf>,
    firefox: Option<PathBuf>,
    safari: Option<PathBuf>,
    opera: Option<PathBuf>,
    brave: Option<PathBuf>,
    edge: Option<PathBuf>,
    vivaldi: Option<PathBuf>,
    chromium: Option<PathBuf>,
}

impl BrowserPaths {
    fn new() -> Self {
        let home = dirs::home_dir();

        BrowserPaths {
            arc: home.as_ref().and_then(Self::arc_path),
            chrome: home.as_ref().and_then(Self::chrome_path),
            firefox: home.as_ref().and_then(Self::firefox_path),
            safari: home.as_ref().and_then(Self::safari_path),
            opera: home.as_ref().and_then(Self::opera_path),
            brave: home.as_ref().and_then(Self::brave_path),
            edge: home.as_ref().and_then(Self::edge_path),
            vivaldi: home.as_ref().and_then(Self::vivaldi_path),
            chromium: home.as_ref().and_then(Self::chromium_path),
        }
    }

    fn arc_path(home: &PathBuf) -> Option<PathBuf> {
        match std::env::consts::OS {
            "macos" => Some(home.join("Library/Application Support/Arc/User Data/Default/History")),
            "windows" => Some(home.join("AppData/Local/Arc/User Data/Default/History")),
            "linux" => Some(home.join(".config/Arc/User Data/Default/History")),
            _ => None,
        }
    }

    fn chrome_path(home: &PathBuf) -> Option<PathBuf> {
        match std::env::consts::OS {
            "macos" => Some(home.join("Library/Application Support/Google/Chrome/Default/History")),
            "windows" => Some(home.join("AppData/Local/Google/Chrome/User Data/Default/History")),
            "linux" => Some(home.join(".config/google-chrome/Default/History")),
            _ => None,
        }
    }

    fn firefox_path(home: &PathBuf) -> Option<PathBuf> {
        match std::env::consts::OS {
            "macos" => {
                Some(home.join("Library/Application Support/Firefox/Profiles/**/places.sqlite"))
            }
            "windows" => {
                Some(home.join(
                    "AppData/Roaming/Mozilla/Firefox/Profiles/*.default-release/places.sqlite",
                ))
            }
            "linux" => Some(home.join(".mozilla/firefox/*.default-release/places.sqlite")),
            _ => None,
        }
    }

    fn safari_path(home: &PathBuf) -> Option<PathBuf> {
        if std::env::consts::OS == "macos" {
            Some(home.join("Library/Safari/History.db"))
        } else {
            None
        }
    }

    fn opera_path(home: &PathBuf) -> Option<PathBuf> {
        match std::env::consts::OS {
            "macos" => {
                Some(home.join("Library/Application Support/com.operasoftware.Opera/History"))
            }
            "windows" => Some(home.join("AppData/Roaming/Opera Software/Opera Stable/History")),
            "linux" => Some(home.join(".config/opera/History")),
            _ => None,
        }
    }

    fn brave_path(home: &PathBuf) -> Option<PathBuf> {
        match std::env::consts::OS {
            "macos" => {
                Some(home.join(
                    "Library/Application Support/BraveSoftware/Brave-Browser/Default/History",
                ))
            }
            "windows" => Some(
                home.join("AppData/Local/BraveSoftware/Brave-Browser/User Data/Default/History"),
            ),
            "linux" => Some(home.join(".config/BraveSoftware/Brave-Browser/Default/History")),
            _ => None,
        }
    }

    fn edge_path(home: &PathBuf) -> Option<PathBuf> {
        match std::env::consts::OS {
            "macos" => {
                Some(home.join("Library/Application Support/Microsoft Edge/Default/History"))
            }
            "windows" => Some(home.join("AppData/Local/Microsoft/Edge/User Data/Default/History")),
            "linux" => Some(home.join(".config/microsoft-edge/Default/History")),
            _ => None,
        }
    }

    fn vivaldi_path(home: &PathBuf) -> Option<PathBuf> {
        match std::env::consts::OS {
            "macos" => Some(home.join("Library/Application Support/Vivaldi/Default/History")),
            "windows" => Some(home.join("AppData/Local/Vivaldi/User Data/Default/History")),
            "linux" => Some(home.join(".config/vivaldi/Default/History")),
            _ => None,
        }
    }

    fn chromium_path(home: &PathBuf) -> Option<PathBuf> {
        match std::env::consts::OS {
            "macos" => Some(home.join("Library/Application Support/Chromium/Default/History")),
            "windows" => Some(home.join("AppData/Local/Chromium/User Data/Default/History")),
            "linux" => Some(home.join(".config/chromium/Default/History")),
            _ => None,
        }
    }
}

const DEFAULT_QUERY: &str = "SELECT id, url, title, visit_count, CAST(((CAST(last_visit_time as REAL) - 11644473600000000) / 1000000) as BIGINT) as last_visit_time FROM urls ORDER BY visit_count DESC";

pub fn get_history_schemas() -> Vec<HistorySchema> {
    let browser_paths = BrowserPaths::new();

    let possible_browser_histories = vec![
        (browser_paths.chrome, DEFAULT_QUERY, Browser::Chrome),
        (browser_paths.arc, DEFAULT_QUERY, Browser::Arc),
        (browser_paths.brave, DEFAULT_QUERY, Browser::Brave),
        (browser_paths.firefox, "SELECT id, url, COALESCE(title, \"\"), visit_count, COALESCE(CAST(last_visit_date / 1000000) as BIGINT), 0) FROM moz_places ORDER BY visit_count DESC", Browser::Firefox),
        (browser_paths.safari, "SELECT i.id, i.url, COALESCE(v.title, \"\"), i.visit_count, CAST(v.visit_time + 978307200 as BIGINT) as visit_time FROM history_items i LEFT JOIN history_visits v ON i.id = v.history_item ORDER BY i.visit_count DESC", Browser::Safari)
    ];

    possible_browser_histories
        .iter()
        .filter_map(|(path, query, browser)| {
            if let Some(p) = path {
                if p.exists() {
                    Some(HistorySchema {
                        browser: browser.to_owned(),
                        path: p.to_string_lossy().to_string(),
                        query: query.to_owned().to_owned(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

// if a browser is running you cannot read the history sqlite directly
// because it's locked. You have to copy it somewhere else and use that copy instead
fn copy_browser_sqlite_to_tmpdir(from: &str, name: &str) {
    let mut dest_path = env::temp_dir();
    dest_path.push(format!("sf-history-{}.sqlite", name));
    if let Err(e) = fs::copy(from, &dest_path) {
        eprintln!("Error: {:?}", e);
    } else {
        println!("Copied to {:?}", dest_path);
    }
}

fn get_copied_sqlite_paths_for_history_schema(schema: &HistorySchema) -> Vec<String> {
    let pattern = format!("sf-history-{}*.sqlite", schema.browser.to_str());
    let mut tmp_path = env::temp_dir();
    tmp_path.push(pattern);

    let glob_pattern = tmp_path.to_str().unwrap_or_else(|| panic!("Invalid path"));

    glob(glob_pattern)
        .expect("error finding sqlite globs in tmp dir")
        .filter_map(Result::ok)
        .map(|path| path.to_string_lossy().to_string())
        .collect()
}

async fn cache_favicons(history: &[HistoryEntry]) {
    for entry in history {
        match Url::parse(&entry.url) {
            Ok(url) => {
                if let Some(domain) = url.domain() {
                    let domain_owned = domain.to_string();
                    cache_favicon(&domain_owned).await;
                }
            }
            Err(e) => {
                println!("Error parsing url: {:?}", e);
            }
        }
    }
}

async fn cache_favicon(domain: &str) {
    match get_favicon_cache_path() {
        None => return,
        Some(mut path) => {
            path.push(format!("{}.png", domain));
            if path.exists() {
                return;
            }
            let request_path = format!("https://t1.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=https://{}&size=64", domain);
            println!("Requesting favicon for '{:?}'", domain);
            if let Ok(icon) = reqwest::get(request_path).await {
                if let Ok(icon) = icon.bytes().await {
                    if let Ok(mut file) = fs::File::create(&path) {
                        if let Err(e) = file.write(&icon) {
                            println!("Error writing favicon to file: {:?}", e);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum BrowserHistoryCollationError {
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

pub struct BrowserHistoryDataSource {
    name: String,
    sqlite: SQLite,
}

impl BrowserHistoryDataSource {
    pub fn read(&self) -> Option<Vec<HistoryEntry>> {
        let query_statement = format!(
            r#"SELECT * FROM {}
                WHERE last_visit_time >= strftime('%s', 'now', '-6 months')
                ORDER BY frecency_score DESC
                LIMIT 1000"#,
            self.name
        );

        self.sqlite
            .conn
            .prepare(&query_statement)
            .map_err(|e| {
                println!("Error reading from history DB: {}", e);
                BrowserHistoryCollationError::UnableToQueryCollatedDB
            })
            .ok()
            .and_then(|mut statement| {
                statement
                    .query_map([], |row| {
                        Ok(HistoryEntry {
                            browser: Browser::from_string(row.get("browser")?),
                            url: row.get("url")?,
                            title: row.get("title")?,
                            visit_count: row.get("visit_count")?,
                            last_visit_time: row.get("last_visit_time")?,
                            frecency_score: row.get("frecency_score")?,
                        })
                    })
                    .ok()
                    .map(|entries| entries.filter_map(Result::ok).collect::<Vec<_>>())
            })
    }
}

impl DataSource<Vec<HistoryEntry>> for BrowserHistoryDataSource {
    fn new(name: &str) -> Self {
        if let Ok(sqlite) = SQLite::new(name, false) {
            let transaction = format!(
                "CREATE TABLE IF NOT EXISTS {} (
                  browser TEXT NOT NULL,
                  url TEXT PRIMARY KEY NOT NULL UNIQUE,
                  title TEXT NOT NULL,
                  visit_count INTEGER NOT NULL,
                  last_visit_time INTEGER NOT NULL,
                  frecency_score REAL NOT NULL
                )",
                name
            );
            if let Err(e) = sqlite.conn.execute(&transaction, []) {
                eprintln!(
                    "failed to create the table '{}', maybe it already exists?\n{:?}",
                    name, e
                )
            };
            Self {
                sqlite,
                name: name.to_string(),
            }
        } else {
            panic!("Error initializing the BrowserHistoryDataSource")
        }
    }

    fn update_cache(&mut self) {
        let schemas: Vec<HistorySchema> = get_history_schemas();

        schemas.iter().for_each(|schema| {
            if schema.path.contains("*") {
                for (i, entry) in glob(&schema.path)
                    .expect("Failed to read glob pattern")
                    .enumerate()
                {
                    match entry {
                        Ok(path) => {
                            copy_browser_sqlite_to_tmpdir(
                                path.to_str().unwrap(),
                                &format!("{}-{}", &schema.browser.to_str(), i + 1),
                            );
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
            } else {
                copy_browser_sqlite_to_tmpdir(&schema.path, schema.browser.to_str());
            }
        });

        let schema_with_paths: Vec<(HistorySchema, Vec<String>)> = schemas
            .iter()
            .map(|schema| {
                (
                    schema.to_owned(),
                    get_copied_sqlite_paths_for_history_schema(schema),
                )
            })
            .collect();

        let entries: Vec<HistoryEntry> = schema_with_paths
            .par_iter()
            .flat_map(|(schema, paths)| {
                paths.par_iter().filter_map(|path| {
                    SQLite::from_path(path, true)
                        .map_err(|e| {
                            eprintln!("Error connecting to db: {:?}", e);
                        })
                        .ok()
                        .and_then(|sqlite| {
                            sqlite.conn.prepare(schema.query.as_str()).ok().and_then(
                                |mut statement| {
                                    statement
                                        .query_map([], |row| {
                                            let mut entry = HistoryEntry {
                                                browser: schema.browser,
                                                url: row.get_unwrap(1),
                                                title: row.get_unwrap(2),
                                                visit_count: row.get_unwrap(3),
                                                last_visit_time: row.get_unwrap(4),
                                                frecency_score: 0.0,
                                            };
                                            entry.frecency_score = calculate_frecency(&entry);

                                            if let Ok(mut url) = Url::parse(entry.url.as_str()) {
                                                url.set_query(None);
                                                url.set_fragment(None);
                                                entry.url = url
                                                    .as_str()
                                                    .trim_end_matches(&['/', '#', '?', '&'])
                                                    .to_string()
                                            }
                                            Ok(entry)
                                        })
                                        .ok()
                                        .map(|rows| {
                                            rows.filter_map(Result::ok)
                                                .filter(|row| {
                                                    !row.url.contains("localhost")
                                                        && !row.url.contains("127.0.0.1")
                                                        && !row.url.contains("0.0.0.0")
                                                        && !row.url.contains("file://")
                                                        && row.frecency_score > 0.0
                                                })
                                                .collect::<Vec<HistoryEntry>>()
                                        })
                                },
                            )
                        })
                })
            })
            .flatten()
            .collect::<Vec<_>>();

        let cacheable = entries.clone();
        spawn(async move {
            cache_favicons(&cacheable).await;
        });

        if let Ok(transaction) = self.sqlite.conn.transaction() {
            let result: Result<(), rusqlite::Error> = (|| {
                match transaction.prepare(
                    &format!("INSERT OR IGNORE INTO {} (browser, url, title, visit_count, last_visit_time, frecency_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", self.name),
                ) {
                  Ok(mut statement) => {
                    for entry in entries {
                        statement.execute(params![
                            entry.browser.to_str(),
                            entry.url,
                            entry.title,
                            entry.visit_count,
                            entry.last_visit_time,
                            entry.frecency_score
                        ])?;
                    }
                  },
                  Err(e) => eprintln!("Error writing history entry: {:?}", e)
                };

                Ok(())
            })();

            match result {
                Ok(_) => {
                    if let Err(e) = transaction.commit() {
                        eprintln!("Error committing browser history to collated DB: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error inserting browser history into collated DB: {:?}", e);
                    // Optionally, you might want to roll back the transaction here
                    // if let Err(rollback_err) = transaction.rollback() {
                    //     eprintln!("Error rolling back transaction: {:?}", rollback_err);
                    // }
                }
            }
        }
    }

    fn query(&self, query: &Query) -> Option<Vec<HistoryEntry>> {
        self.read()
            .map(|entries| {
                let search_string = query.search_string.clone();
                let start = Instant::now();

                let matcher = SkimMatcherV2::default().ignore_case();
                let mut scored_entries: Vec<(i64, HistoryEntry)> = entries
                    .par_iter()
                    .filter_map(|entry| {
                        let url = entry.url.clone();
                        let title = entry.title.clone();

                        let mut score = matcher
                            .fuzzy(&url, &search_string, false)
                            .map(|res| res.0)
                            .unwrap_or(0);

                        let title_score = matcher
                            .fuzzy(&title, &search_string, true)
                            .map(|res| res.0)
                            .unwrap_or(0);

                        score = cmp::max(score, title_score);
                        if score > 0 {
                            Some((score, entry.to_owned()))
                        } else {
                            None
                        }
                    })
                    .collect();
                println!(
                    "finished browser history search in {}ms",
                    start.elapsed().as_millis()
                );

                scored_entries.sort_by(|a, b| b.0.cmp(&a.0));
                Some(
                    scored_entries
                        .iter()
                        .take(50)
                        .map(|res| res.1.clone())
                        .collect(),
                )
            })
            .unwrap_or(None)
    }
}
