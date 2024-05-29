// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beijing_bus_transfer_system::{dbaccess, entities::AppState};
use tauri::{AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    let db_path = app_handle
        .path_resolver()
        .resolve_resource("_up_/bus-data/bus.db")
        .expect("failed to resolve resource");
    let _ = dbaccess::connect_db(db_path);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 获取db路径并存入APP托管状态
            let app_handle = app.handle();
            let db_path = app_handle.path_resolver()
                .resolve_resource("_up_/bus-data/bus.db")
                .expect("failed to resolve resource");
            let mut app_state = AppState::new(db_path);
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
