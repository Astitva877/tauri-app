// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) {
    format!("Hello, {}! You've been greeted from Rust!", name);
}
#[derive(Clone, serde::Serialize)]
struct Payload {
    url: String,
}
#[derive(Clone, serde::Serialize)]
struct PayloadTwo {
    args: Vec<String>,
    cwd: String,
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            app.emit(
                "deep-link-urls",
                Payload {
                    url: args[1].to_string(),
                },
            )
            .unwrap();
            app.emit(
                "single-instance",
                PayloadTwo {
                    args,
                    cwd,
                },
            )
            .unwrap();
        }))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
