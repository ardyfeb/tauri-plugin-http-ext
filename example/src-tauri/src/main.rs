#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_plugin_http_ext as http_ext;

fn main() {
    let config = tauri_plugin_http_ext::ClientConfig {
        tls: None
    };

    tauri::Builder::default()
        .plugin(http_ext::init(config))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
