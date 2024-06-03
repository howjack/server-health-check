pub mod structs;

use rusqlite::{Connection};
use tauri::AppHandle;
use std::fs;

const CURRENT_DB_VERSION: u32 = 1;

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
	let app_dir = app_handle.path_resolver().app_data_dir().expect("The app data directory should exist.");
	fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
	let sqlite_path = app_dir.join("server_health_check.sqlite");

	let mut db = Connection::open(sqlite_path)?;

	let mut user_pragma = db.prepare("PRAGMA user_version")?;
	let existing_user_version: u32 = user_pragma.query_row([], |row| { Ok(row.get(0)?) })?;
	drop(user_pragma);

	upgrade_database_if_needed(&mut db, existing_user_version)?;

	Ok(db)
}

pub fn upgrade_database_if_needed(db: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
	if existing_version < CURRENT_DB_VERSION {
		db.pragma_update(None, "journal_mode", "WAL")?;
		db.pragma_update(None, "foreign_keys", "ON")?;

		let tx = db.transaction()?;

		tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

		tx.execute_batch(
			"
	      CREATE TABLE apis
				(
				    id        INTEGER PRIMARY KEY,
				    name      TEXT NOT NULL,
				    url       TEXT NOT NULL,
				    method    TEXT,
				    color_hex TEXT,
				    status    INTEGER DEFAULT 1,
				    created   TEXT NOT NULL DEFAULT (datetime('now'))
				);

				CREATE TABLE monitoring
				(
				    id          INTEGER PRIMARY KEY,
				    api_id      INTEGER NOT NULL,
				    status_code TEXT    NOT NULL,
				    time_length REAL,
				    status      INTEGER          DEFAULT 1,
				    created     TEXT    NOT NULL DEFAULT (datetime('now')),
				    FOREIGN KEY (api_id)
				        REFERENCES apis (id)
				);

				CREATE TABLE settings
				(
				    id                       INTEGER PRIMARY KEY,
				    api_id                   INTEGER UNIQUE NOT NULL,
				    monitoring_interval_time REAL,
				    notification_error_count INTEGER,
				    notification_time_length REAL,
				    created                  TEXT    NOT NULL DEFAULT (datetime('now')),
				    FOREIGN KEY (api_id)
				        REFERENCES apis (id)
				);

				CREATE TABLE configurations
				(
				    id                       INTEGER PRIMARY KEY,
				    monitoring_interval_time REAL    NOT NULL,
				    notification_error_count INTEGER NOT NULL,
				    notification_time_length REAL    NOT NULL,
				    created                  TEXT    NOT NULL DEFAULT (datetime('now'))
				);
			")?;

		tx.commit()?;
	}

	Ok(())
}


