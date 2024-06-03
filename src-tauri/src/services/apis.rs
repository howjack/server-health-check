use rusqlite::{Connection, named_params, OptionalExtension};

use crate::database::structs;

// ------ APIs ------
pub fn add_api(db: &Connection, data: structs::AddAPI) -> Result<(), rusqlite::Error> {
	let mut statement = db.prepare(
		"INSERT INTO apis (name, url, status)	VALUES (@name, @url, @status)"
	)?;
	statement.execute(named_params! {
		"@name": data.name,
		"@url": data.url,
		"@status": data.status
	})?;

	Ok(())
}

pub fn list_apis(db: &Connection) -> Result<Vec<structs::API>, rusqlite::Error> {
	let mut statement = db.prepare("SELECT * FROM apis")?;
	let mut rows = statement.query([])?;

	let mut items = Vec::new();

	while let Some(row) = rows.next()? {
		let row = structs::API {
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

pub fn get_api_by_id(db: &Connection, id: i32) -> Result<Option<structs::API>, rusqlite::Error> {
	let mut statement = db.prepare("SELECT * FROM apis where id = @id")?;
	let row = statement.query_row(
		named_params! { "@id": id },
		|r| Ok(structs::API {
			id: r.get("id")?,
			name: r.get("name")?,
			url: r.get("url")?,
			method: r.get("method")?,
			color_hex: r.get("color_hex")?,
			status: r.get("status")?,
			created: r.get("created")?,
		}),
	).optional()?;

	Ok(row)
}

pub fn edit_api(db: &Connection, data: structs::EditAPI) -> Result<(), rusqlite::Error> {
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
		"@id": data.id,
		"@name": data.name,
		"@url": data.url,
		"@status": data.status,
		"@color_hex": data.color_hex,
		"@method": data.method
	})?;

	Ok(())
}

pub fn delete_api(db: &Connection, id: i32) -> Result<(), rusqlite::Error> {
	let mut statement = db.prepare("DELETE FROM apis WHERE id = @id;")?;
	statement.execute(named_params! { "@id": id })?;

	Ok(())
}