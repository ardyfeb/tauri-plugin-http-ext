#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    // let config = tauri_plugin_mtls::ClientConfig {
    //     ca: include_bytes!("/Users/ardyfeb/Documents/certs/ardyfeb.dev/ardyfeb.dev.crt"),
    //     cert: include_bytes!("/Users/ardyfeb/Documents/certs/ardyfeb.dev/ardyfeb.dev.1.pem"),
    // };
    let config = tauri_plugin_mtls::ClientConfig {
        ca: include_bytes!("/Users/ardyfeb/Documents/certs/local/client_0.crt"),
        cert: include_bytes!("/Users/ardyfeb/Documents/certs/local/client_0.pem"),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_mtls::init(config))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
