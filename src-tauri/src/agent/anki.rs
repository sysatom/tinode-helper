use std::error::Error;
use std::fmt;
use serde::{Deserialize};
use serde_json::{json, Value};
use crate::agent::agent_post_data;
use crate::agent::AGENT_VERSION;

pub fn run() {}

const API_VERSION: i32 = 6;
const API_URI: &str = "http://127.0.0.1:8765";

const URI: &str = "";//todo

#[derive(Deserialize)]
struct Response {
    error: Option<String>,
    result: Value,
}

async fn stats() {
    let result = get_collection_stats_html().await;
    match result {
        Ok(html) => {
            agent_post_data(URI.to_string(), json!({
                "action": "agent",
                "version": AGENT_VERSION,
                "content": {
                    "id": "stats_agent",
                    "content": {
                        "html": html,
                    }
                }
            })).await.unwrap_or(false);
        }
        Err(err) => println!("{}", err),
    }
}

async fn review() {
    let result = get_num_cards_reviewed_today().await;
    match result {
        Ok(count) => {
            agent_post_data(URI.to_string(), json!({
                "action": "agent",
                "version": AGENT_VERSION,
                "content": {
                    "id": "review_agent",
                    "content": {
                        "num": count,
                    }
                }
            })).await.unwrap_or(false);
        }
        Err(err) => println!("{}", err),
    }
}

async fn get_collection_stats_html() -> Result<String, Box<dyn Error>> {
    let param = json!({
        "action":  "getCollectionStatsHTML",
        "version": API_VERSION,
        "params": {
            "wholeCollection": true,
        }
    });

    let client = reqwest::Client::new();
    let response = client.post(API_URI)
        .body(param.to_string())
        .send()
        .await?;

    if response.status().is_success() {
        let result = response.json::<Response>().await?;
        if let Some(error) = result.error {
            Err(error.into())
        } else {
            let html = String::from(result.result.as_str().unwrap());
            Ok(html)
        }
    } else {
        Err("error response".into())
    }
}

async fn get_num_cards_reviewed_today() -> Result<i64, Box<dyn Error>> {
    let param = json!({
       "action":  "getNumCardsReviewedToday",
        "version": API_VERSION,
    });

    let client = reqwest::Client::new();
    let response = client.post(API_URI)
        .body(param.to_string())
        .send()
        .await?;

    if response.status().is_success() {
        let result = response.json::<Response>().await?;
        if let Some(error) = result.error {
            Err(error.into())
        } else {
            Ok(result.result.as_i64().unwrap_or(-1))
        }
    } else {
        Err("error response".into())
    }
}

#[cfg(test)]
mod test {
    use crate::agent::anki::{get_collection_stats_html, get_num_cards_reviewed_today, review, stats};

    #[tokio::test]
    async fn test_get_collection_stats_html() {
        let result = get_collection_stats_html().await.unwrap();
        assert!(result.len() > 0)
    }

    #[tokio::test]
    async fn test_stats() {
        stats().await;
    }

    #[tokio::test]
    async fn test_get_num_cards_reviewed_today() {
        let result = get_num_cards_reviewed_today().await.unwrap();
        assert_eq!(result, 0)
    }

    #[tokio::test]
    async fn test_review() {
        review().await;
    }
}