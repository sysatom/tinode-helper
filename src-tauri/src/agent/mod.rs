use log::info;

pub mod anki;

pub const AGENT_VERSION: i32 = 1;

async fn agent_post_data(agent_uri: String, data: serde_json::Value) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.post(agent_uri)
        .body(data.to_string())
        .send()
        .await?;

    info!("agent_post_data request status {}", response.status());

    if response.status().is_success() {
        Ok(true)
    } else {
        Err("error agent post data".into())
    }
}