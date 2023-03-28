use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use arboard::Clipboard;
use log::info;
use tauri::{App, Wry};
use tauri::ClipboardManager;

pub(crate) struct AppCtx {
    path: String,
}

impl AppCtx {
    pub(crate) fn new(path: String) -> Self {
        Self { path }
    }

    pub(crate) async fn get_clipboard(&self) -> String {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.get_text().unwrap()
    }
}
