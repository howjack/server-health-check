use rusqlite::{Connection, named_params};
use tauri::AppHandle;
use std::fs;

use serde::{Serialize, Deserialize};

const CURRENT_DB_VERSION: u32 = 1;

// ---- database initializer ----
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

// ---- database structs ----
#[derive(Serialize)]
pub struct API {
	id: i32,
	name: String,
	url: String,
	method: Option<String>,
	color_hex: Option<String>,
	status: Option<bool>,
	created: String,
}

#[derive(Deserialize)]
pub struct AddAPI {
	name: String,
	url: String,
	status: Option<bool>,
}

#[derive(Deserialize)]
pub struct EditAPI {
	id: i32,
	name: Option<String>,
	url: Option<String>,
	method: Option<String>,
	color_hex: Option<String>,
	status: Option<bool>,
}

// ---- Services ----
pub fn add_api(db: &Connection, data: AddAPI) -> Result<(), rusqlite::Error> {
	let mut statement = db.prepare(
		"INSERT INTO apis (name, url, status)	VALUES (@name, @url, @status);"
	)?;
	statement.execute(named_params! {
		"@name": data.name,
		"@url": data.url,
		"@status": data.status
	})?;

	Ok(())
}

pub fn get_apis(db: &Connection) -> Result<Vec<API>, rusqlite::Error> {
	let mut statement = db.prepare("SELECT * FROM apis;")?;
	let mut rows = statement.query([])?;

	let mut items = Vec::new();

	while let Some(row) = rows.next()? {
		let row = API {
			id: row.get("id")?,
			name: row.get("name")?,
			url: row.get("url")?,
			method: row.get("method")?,
			color_hex: row.get("color_hex")?,
			status: row.get("status")?,
			created: row.get("created")?,
		};

		items.push(row)
	}

	Ok(items)
}

pub fn get_apis_by_id(db: &Connection, id: &i32) -> Result<API, rusqlite::Error> {
	let mut statement = db.prepare("SELECT * FROM apis where id = @id;")?;
	let mut row = statement.query_row(named_params! { "@id": id }, ())?;

	let row = API {
		id: row.get("id")?,
		name: row.get("name")?,
		url: row.get("url")?,
		method: row.get("method")?,
		color_hex: row.get("color_hex")?,
		status: row.get("status")?,
		created: row.get("created")?,
	};

	Ok(row)
}

pub fn edit_api(db: &Connection, data: EditAPI) -> Result<(), rusqlite::Error> {
	let mut statement = db.prepare(
		"UPDATE apis
				SET name      = @name,
				    url       = @url,
				    status    = @status,
				    color_hex = @color_hex,
				    method    = @method
				WHERE id = @id;"
	)?;
	statement.execute(named_params! {
		"@name": data.name,
		"@url": data.url,
		"@status": data.status,
		"@color_hex": data.color_hex,
		"@method": data.method
	})?;

	Ok(())
}

pub fn delete_api(db: &Connection, id: &i32) -> Result<(), rusqlite::Error> {
	let mut statement = db.prepare("DELETE FROM apis WHERE id = @id;")?;
	statement.execute(named_params! { "@id": id })?;

	Ok(())
}