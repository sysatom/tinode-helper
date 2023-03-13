use std::collections::HashMap;

use reqwest;
use tauri;
use serde_json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn demo() {
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:8765")
        .body(r#"{"action": "deckNames", "version": 6}"#)
        .send()
        .await.expect("error post")
        .json::<HashMap<String, serde_json::Value>>()
        .await.expect("error json");

    println!("{res:?}");
}
