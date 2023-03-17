use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use chrono::prelude::*;
use serde_json::Value;
use tauri::api::path::data_dir;
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;

pub async fn setup(app: tauri::AppHandle) {
    let sched = JobScheduler::new().await;
    let sched = sched.unwrap();

    let app_arc = Arc::new(app);
    let job = Job::new("1/5 * * * * *", move |_, _| {
        let store_path = data_dir().unwrap().join("helper").join("store.data");
        let mut store = StoreBuilder::new(app_arc.clone().app_handle(), store_path).build();
        store.load().unwrap();
        let val = store.get("access-url".to_string());
        let default_val = Value::String("abc".to_string());
        println!("every 5 sec, {} {}", Local::now(), val.unwrap_or(&default_val));
    }).unwrap();


    sched.add(job).await.unwrap();

    let start = sched.start().await;
    if start.is_err() {
        println!("scheduler start error");
    }
}