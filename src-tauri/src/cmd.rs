use tauri::api::path::data_dir;
use tauri;

#[tauri::command]
pub fn set_review_count(count: &str) {
    let mut rev_count = count.to_string();
    rev_count.insert_str(0, " ");
}

#[tauri::command]
pub fn get_store_path() -> String {
    let store_path = data_dir().unwrap().join("helper").join("store.data");
    return store_path.to_str().unwrap().to_string();
}
