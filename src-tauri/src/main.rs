// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;

use tauri::{App, Manager};

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut App| {
            setup(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmds::greet, cmds::demo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) {
    let window = app.get_window("main").unwrap();
    println!("{}", window.label());

    // todo config
}
