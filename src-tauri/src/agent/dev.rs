use std::error::Error;
use log::info;
use serde_json::json;
use crate::agent::{agent_post_data, AGENT_VERSION, URI};

pub const NAME: &str = "dev";

pub fn instruct(flag: &str) {
    match flag {
        "help_example" => {
            info!("help_instruct...")
        }
        _ => {
            info!("help default instruct");
        }
    }
}

async fn example() -> Result<(), Box<dyn Error>> {
    agent_post_data(URI.to_string(), json!({
        "action": "agent",
        "version": AGENT_VERSION,
        "content": {
            "id": "import_agent",
            "content": {
                "txt": "example",
            }
        }
    })).await?;
    Ok(())
}