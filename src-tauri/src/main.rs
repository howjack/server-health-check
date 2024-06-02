// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use std::option::Option;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

#[tauri::command]
fn add_apis(app_handle: AppHandle, data: database::AddAPI) -> bool {
	app_handle.db(|db| database::add_api(db, data)).unwrap();

	return true;
}

#[tauri::command]
fn list_apis(app_handle: AppHandle) -> Vec<database::API> {
	let items = app_handle.db(|db| database::list_apis(db)).unwrap();

	return items;
}

#[tauri::command]
fn get_api_by_id(app_handle: AppHandle, id: i32) -> Result<Option<database::API>, ()> {
	let item = app_handle.db(|db| database::get_api_by_id(db, id)).unwrap();

	return Ok(item)
}

#[tauri::command]
fn edit_api(app_handle: AppHandle, data: database::EditAPI) -> bool {
	app_handle.db(|db| database::edit_api(db, data)).unwrap();

	return true;
}

#[tauri::command]
fn delete_api(app_handle: AppHandle, id: i32) -> bool {
	app_handle.db(|db| database::delete_api(db, id)).unwrap();

	return true;
}

fn main() {
	tauri::Builder::default()
		.manage(AppState { db: Default::default() })
		.invoke_handler(tauri::generate_handler![add_apis, list_apis, get_api_by_id, edit_api, delete_api])
		.setup(|app| {
			let handle = app.handle();

			let app_state: State<AppState> = handle.state();
			let db = database::initialize_database(&handle).expect("Database initialize should succeed");
			*app_state.db.lock().unwrap() = Some(db);

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
