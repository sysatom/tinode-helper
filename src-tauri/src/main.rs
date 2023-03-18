#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod cmd;
mod scheduler;
mod agent;

use std::sync::Arc;
use env_logger::{Builder, Env};
use env_logger::fmt::Color;
use log::{info, Level};
use tauri::Manager;
use tauri::{CustomMenuItem, PhysicalPosition, SystemTray, SystemTrayEvent, SystemTrayMenu};
use crate::agent::anki;
use std::io::Write;


fn init_logger() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info");

    Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            // style.set_bg(Color::Yellow).set_color(Color::White).set_bold(true);

            let timestamp = buf.timestamp();

            writeln!(
                buf,
                "helper log ({}): {}",
                timestamp,
                style.value(record.args())
            )
        })
        .init();
}

fn main() {
    // log
    init_logger();

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
            let window = app.get_window("main").unwrap();
            window.set_always_on_top(true).unwrap();

            // cron
            // let app_handle = Arc::new(app.handle());
            tauri::async_runtime::spawn(async move {
                info!("cron starting");
                anki::do_something().await;
                // scheduler::setup(app_handle.clone().app_handle()).await;
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
