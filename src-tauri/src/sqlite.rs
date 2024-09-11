use std::path::PathBuf;

use rusqlite::{Connection, MAIN_DB};
use swordfish_utilities::get_data_path;

pub struct SQLite {
    pub conn: Connection,
}

impl SQLite {
    pub fn new(database_name: &str, readonly: bool) -> Result<Self, &'static str> {
        if let Some(mut dir) = get_data_path() {
            dir.push(&format!("{}.sqlite", &database_name));
            let conn = match Connection::open(dir) {
                Ok(conn) => conn,
                Err(_) => {
                    return Err("Couldn't create the db connection");
                }
            };

            SQLite::set_pragma(&conn, readonly);
            Ok(Self { conn })
        } else {
            Err("Failed to locate data directory")
        }
    }

    fn set_pragma(conn: &Connection, readonly: bool) {
        let db = Some(MAIN_DB);

        // enables write-ahead log so that reads do not block writes and vice-versa.
        conn.pragma_update(db, "journal_mode", "WAL").ok();

        // wait 5 seconds to obtain a lock before returning SQLITE_BUSY errors, which will significantly reduce them.
        conn.pragma_update(db, "busy_timeout", 5000).ok();

        // sync less frequently and be more performant, still safe to use because of the enabled WAL mode.
        conn.pragma_update(db, "synchronous", "NORMAL").ok();

        // negative number means kilobytes, in this case 20MB of memory for cache.
        conn.pragma_update(db, "cache_size", -20000).ok();

        // for historical reasons foreign keys are disabled by default.
        conn.pragma_update(db, "foreign_keys", true).ok();

        // moves temporary tables from disk into RAM, speeds up performance a lot.
        conn.pragma_update(db, "temp_store", "memory").ok();

        // Do NOT use cache=shared! Some tutorials recommend configuring it, but this is how you get nasty SQLITE_BUSY errors. It is disabled by default, so you don't have to do anything extra.
        // If you know that transaction can possibly do a write, always use BEGIN IMMEDIATE or you can a get SQLITE_BUSY error. Check your framework,  you should be able to set this at the connection level.

        if readonly {
            conn.pragma_update(db, "mode", "ro").ok();
        } else {
            conn.pragma_update(db, "txlock", "IMMEDIATE").ok();
            conn.pragma_update(db, "mode", "rwc").ok();
        }
    }

    pub fn from_path(path: &str, readonly: bool) -> Result<Self, &'static str> {
        let filepath = PathBuf::from(path);
        if filepath.is_file() {
            let conn = match Connection::open(filepath) {
                Ok(conn) => conn,
                Err(_) => {
                    return Err("Couldn't create the db connection");
                }
            };

            SQLite::set_pragma(&conn, readonly);
            Ok(Self { conn })
        } else {
            Err("Failed to locate data directory")
        }
    }
}
