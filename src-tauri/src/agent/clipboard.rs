use std::error::Error;
use log::info;
use serde_json::json;
use crate::agent::{agent_post_data, AGENT_VERSION, URI};

pub const NAME: &str = "clipboard";

pub fn instruct(flag: &str) {
    match flag {
        "clipboard_share" => {
            info!("share clipboard")
        }
        _ => {
            info!("clipboard default instruct");
        }
    }
}

async fn upload() -> Result<(), Box<dyn Error>> {
    agent_post_data(URI.to_string(), json!({
        "action": "agent",
        "version": AGENT_VERSION,
        "content": {
            "id": "clipboard_upload",
            "content": {
                "txt": "example",
            }
        }
    })).await?;
    Ok(())
}