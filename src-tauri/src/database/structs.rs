use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct API {
	pub id: i32,
	pub name: String,
	pub url: String,
	pub method: Option<String>,
	pub color_hex: Option<String>,
	pub status: Option<bool>,
	pub created: String,
}

#[derive(Deserialize)]
pub(crate) struct AddAPI {
	pub name: String,
	pub url: String,
	pub status: Option<bool>,
}

#[derive(Deserialize)]
pub struct EditAPI {
	pub id: i32,
	pub name: Option<String>,
	pub url: Option<String>,
	pub method: Option<String>,
	pub color_hex: Option<String>,
	pub status: Option<bool>,
}
