// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use commands::{create_task_cmd, get_all_task_cmd};
use tauri::generate_handler;

mod app_state;
mod commands;
mod config;
mod message;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![create_task_cmd, get_all_task_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
