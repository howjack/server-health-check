// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

#[tauri::command]
fn add_apis(app_handle: AppHandle, data: database::AddAPI) -> bool {

	app_handle.db(|db| database::add_api(db, data)).unwrap();

	return true
}

#[tauri::command]
fn list_apis(app_handle: AppHandle) -> Vec<database::API> {

	let items = app_handle.db(|db| database::get_apis(db)).unwrap();

	return items
}

#[tauri::command]
fn edit_api(app_handle: AppHandle, data: database::AddAPI) -> Vec<database::API> {

}

fn main() {
	tauri::Builder::default()
		.manage(AppState { db: Default::default() })
		.invoke_handler(tauri::generate_handler![add_apis, list_apis])
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
