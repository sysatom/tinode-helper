use std::error::Error;
use std::thread;
use std::fmt::Display;
use std::time::Duration;
use log::{error, info};
use serde::{Deserialize};
use serde_json::{json, Value};
use crate::agent::{agent_post_data, URI, AGENT_VERSION};
use anyhow::{anyhow, Result};
use delay_timer::prelude::*;
use delay_timer::timer::timer_core::Timer;

pub const NAME: &str = "anki";

const API_VERSION: i32 = 6;
const API_URI: &str = "http://127.0.0.1:8765";

pub async fn do_something() {
    info!("do_something starting");
    loop {
        if let Err(err) = stats().await {
            error!("do_something run: {}", err);
        }
        thread::sleep(Duration::from_secs(5));
    }
}

pub async fn example() -> Result<()> {
    let delay_timer = DelayTimerBuilder::default().build();
    delay_timer.insert_task(build_task_async_print()?)?;
    Ok(())
}

fn build_task_async_print() -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    let body = || async {
        info!("create async fn body");

        thread::sleep(Duration::from_secs(3));

        info!("create async body success");
    };

    task_builder
        .set_task_id(1)
        .set_frequency_repeated_by_cron_str("@secondly")
        .spawn_async_routine(body)
}

pub fn instruct(flag: &str) {
    match flag {
        _ => {
            info!("anki default instruct");
        }
    }
}


#[derive(Deserialize)]
struct Response {
    error: Option<String>,
    result: Value,
}

async fn stats() -> Result<()> {
    let html = get_collection_stats_html().await?;
    agent_post_data(URI.to_string(), json!({
        "action": "agent",
        "version": AGENT_VERSION,
        "content": {
            "id": "stats_agent",
            "content": {
                "html": html,
            }
        }
    })).await?;
    Ok(())
}

async fn review() -> Result<()> {
    let count = get_num_cards_reviewed_today().await?;
    agent_post_data(URI.to_string(), json!({
        "action": "agent",
        "version": AGENT_VERSION,
        "content": {
            "id": "review_agent",
            "content": {
                "num": count,
            }
        }
    })).await?;
    Ok(())
}

async fn get_collection_stats_html() -> Result<String> {
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

    info!("anki getCollectionStatsHTML request status {}", response.status());

    if response.status().is_success() {
        let result = response.json::<Response>().await?;
        if let Some(error) = result.error {
            Err(anyhow!(error))
        } else {
            let html = String::from(result.result.as_str().unwrap());
            Ok(html)
        }
    } else {
        Err(anyhow!("error response"))
    }
}

async fn get_num_cards_reviewed_today() -> Result<i64> {
    let param = json!({
       "action":  "getNumCardsReviewedToday",
        "version": API_VERSION,
    });

    let client = reqwest::Client::new();
    let response = client.post(API_URI)
        .body(param.to_string())
        .send()
        .await?;

    info!("anki getNumCardsReviewedToday request status {}", response.status());

    if response.status().is_success() {
        let result = response.json::<Response>().await?;
        if let Some(error) = result.error {
            Err(anyhow!(error))
        } else {
            Ok(result.result.as_i64().unwrap_or(-1))
        }
    } else {
        Err(anyhow!("error response"))
    }
}


#[derive(Debug)]
struct CustomError {
    message: String,
}

impl CustomError {
    fn new(message: &str) -> CustomError {
        CustomError {
            message: message.to_string(),
        }
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {}

unsafe impl Send for CustomError {}

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
        stats().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_num_cards_reviewed_today() {
        let result = get_num_cards_reviewed_today().await.unwrap();
        assert_eq!(result, 0)
    }

    #[tokio::test]
    async fn test_review() {
        review().await.unwrap();
    }
}