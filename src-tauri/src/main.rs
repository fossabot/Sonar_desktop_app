// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use sonar_desktop_app::{
    cli::print_banner,
    get_interfaces::get_interfaces,
    save_packets::{cmd_save_packets_to_csv, MyError},
    sniff::scan_until_interrupt,
    tauri_state::SonarState,
};
use tauri::{Manager,State};

extern crate sonar_desktop_app;

fn main() {
    println!("{}", print_banner());

    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = devtools::init();

    let builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    let builder = builder.plugin(devtools);

    builder
    .on_window_event(|event| match event.event() {
        tauri::WindowEvent::CloseRequested { .. } => {
          std::process::exit(0);
        }
        _ => {}
      })
        .manage(SonarState(Arc::new(Mutex::new(Vec::new()))))
        .invoke_handler(tauri::generate_handler![
            get_interfaces_tab,
            get_selected_interface,
            save_packets_to_csv,
            get_hash_map_state
        ])
        .setup(move |app| {
            let app_handle = app.handle();

            // Event listener for before-quit
            app_handle.listen_global("tauri://before-quit", move |_| {
                println!("Quit event received");
                
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(rename_all = "snake_case")]
fn get_interfaces_tab() -> Vec<String> {
    get_interfaces()
}

#[tauri::command(async, rename_all = "snake_case")]
fn get_selected_interface(
    window: tauri::Window,
    interface_name: String,
    state: tauri::State<SonarState>,
) {
    let app = window.app_handle();
    println!("{}", &interface_name);
    println!("You have selected the interface: {}", interface_name);

    scan_until_interrupt(app, &interface_name, state);
}

#[tauri::command(async, rename_all = "snake_case")]
fn save_packets_to_csv(file_path: String, state: State<SonarState>) -> Result<(), MyError> {
    println!("{}", &file_path);
    cmd_save_packets_to_csv(file_path, state)
}

#[tauri::command]
fn get_hash_map_state(shared_hash_map: State<SonarState>) -> Result<String, String> {
    // Attempt to acquire the lock on the shared state
    let hash_map = shared_hash_map
        .0
        .lock()
        .map_err(|_| "Failed to lock the mutex")?;

    // Serialize the hash map to a JSON string
    serde_json::to_string(&*hash_map).map_err(|e| format!("Serialization error: {}", e))
}
