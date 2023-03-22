use crate::agent::AGENT_VERSION;
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::thread;
use std::time::Duration;
use log::info;

pub async fn pull() {
    info!("pull starting");
    loop {
        if let Ok(instruct) = get_pull_list().await {
            for item in instruct.iter() {
                info!("instruct -> {:?}", item);
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}

#[derive(Deserialize, Debug)]
struct Instruct {
    no: String,
    bot: String,
    flag: String,
    content: serde_json::Value,
    expire_at: String,
}

#[derive(Deserialize)]
struct Data {
    instruct: Vec<Instruct>,
}

const URI: &str = "http://127.0.0.1:6060/extra/helper/17818979592756537925/YG2ZztWyW54";

async fn get_pull_list() -> Result<Vec<Instruct>, Box<dyn Error>> {
    let param = json!({
        "action": "pull",
        "version": AGENT_VERSION,
    });

    let client = reqwest::Client::new();
    let res = client.post(URI).body(param.to_string()).send().await?;

    info!("pull status {}", res.status());

    let data = res.json::<Data>().await?;
    Ok(data.instruct)
}

#[cfg(test)]
mod test {
    use crate::instruct::{get_pull_list, pull};

    #[tokio::test]
    async fn test_pull() {
        pull().await;
    }

    #[tokio::test]
    async fn test_get_pull_list() {
        get_pull_list().await.unwrap();
    }
}
