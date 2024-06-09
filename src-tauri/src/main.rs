// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beijing_bus_transfer_system::commands::*;
use beijing_bus_transfer_system::entities::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 获取db路径并存入APP托管状态
            let app_handle = app.handle();
            let db_path = app_handle
                .path_resolver()
                .resolve_resource("_up_/bus-data/bus.db")
                .expect("failed to resolve resource");
            let app_state = AppState::new(db_path)?;
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            search_stops_name,
            search_lines_name,
            search_the_shortest_path,
            search_the_min_transfer_path,
            search_the_stops_lines,
            search_the_lines_stops,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
