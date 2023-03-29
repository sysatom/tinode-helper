#![cfg_attr(
   all(not(debug_assertions), target_os = "windows"),
   windows_subsystem = "windows"
)]

use std::process::exit;
use std::sync::{Arc};
use tauri::{ClipboardManager, Manager};
use tauri::{CustomMenuItem, PhysicalPosition, SystemTray, SystemTrayEvent, SystemTrayMenu};
use crate::agent::anki;
use log::{error, info};
use crate::ctx::AppCtx;
use crate::logger::init_logger;
use crate::util::server;

mod cmd;
mod scheduler;
mod agent;
mod instruct;
mod logger;
mod ctx;
mod util;

fn main() {
    // log
    init_logger();

    // check singleton
    if server::check_singleton().is_err() {
        error!("app exists");
        exit(1);
    }

    info!("starting app ...");

    let quit = CustomMenuItem::new("quit".to_string(), "Quit Helper");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .system_tray(system_tray)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::LeftClick { position, size, .. } => {
                let w = app.get_window("main").unwrap();
                let visible = w.is_visible().unwrap();
                if visible {
                    w.hide().unwrap();
                } else {
                    let window_size = w.outer_size().unwrap();
                    let physical_pos = PhysicalPosition {
                        x: position.x as i32 + (size.width as i32 / 2)
                            - (window_size.width as i32 / 2),
                        y: position.y as i32 - window_size.height as i32,
                    };

                    let _ = w.set_position(tauri::Position::Physical(physical_pos));
                    w.show().unwrap();
                    w.set_focus().unwrap();
                }
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // don't kill the app when the user clicks close.
                event.window().hide().unwrap();
                api.prevent_close();
            }
            tauri::WindowEvent::Focused(false) => {
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![cmd::set_review_count, cmd::get_store_path])
        .setup(|app| {
            // embed server for check
            server::embed_server(app.handle());

            let window = app.get_window("main").unwrap();
            window.set_always_on_top(true).unwrap();

            app.clipboard_manager();
            let t = app.handle().clipboard_manager().read_text().unwrap().unwrap();
            info!("clipboard_manager {}", t);

            // cron
            tauri::async_runtime::spawn(async {
                scheduler::setup().await.unwrap();
            });
            tauri::async_runtime::spawn(async move {
                anki::do_something().await;
            });
            tauri::async_runtime::spawn(async move {
                instruct::pull().await;
            });
            tauri::async_runtime::spawn(async {
               anki::example().await.unwrap();
            });

            let app_arc = Arc::new(AppCtx::new("path".into()));
            let app_clone = app_arc.clone();
            tauri::async_runtime::spawn(async move {
                println!("async clipboard {}", app_clone.get_clipboard().await);
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        })
}
