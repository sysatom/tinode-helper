use anyhow::{Result, Ok, bail};
use log::error;
use tauri::AppHandle;
use warp::Filter;
use crate::util::local;

const PORT: u16 = 33331;

pub fn check_singleton() -> Result<()> {
    if !local::port_available(PORT) {
        tauri::async_runtime::block_on(async {
            let url = format!("http://127.0.0.1:{}/commands/visible", PORT);
            let resp = reqwest::get(url).await?.text().await?;

            if &resp == "ok" {
                bail!("app exists");
            }

            error!("failed to setup singletion listen server");
            Ok(())
        })
    } else {
        Ok(())
    }
}

pub fn embed_server(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let commands = warp::path!("commands" / "visible").map(move || {
            format!("ok")
        });
        warp::serve(commands).bind(([127, 0, 0, 1], PORT)).await;
    });
}