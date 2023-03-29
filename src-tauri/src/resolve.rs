use std::sync::Arc;
use log::info;
use tauri::{App, Manager, Runtime};
use crate::agent::anki;
use crate::{instruct, scheduler};
use crate::ctx::AppCtx;
use crate::util::server;

pub fn resolve_setup(app: &mut App) {
    // embed server for check
    server::embed_server(app.handle());

    // main window
    let window = app.get_window("main").unwrap();
    window.set_always_on_top(true).unwrap();

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
}