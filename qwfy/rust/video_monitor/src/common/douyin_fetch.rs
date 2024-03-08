// qwfy/rust/video_monitor/src/common/douyin_fetch.rs

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, COOKIE, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS};
use std::fs;

pub async fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = fs::read_to_string("./config/douyin_config.ini")?;
    let mut headers = HeaderMap::new();

    for line in config.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "user-agent" => headers.insert(USER_AGENT, HeaderValue::from_str(value)?),
                "accept" => headers.insert(ACCEPT, HeaderValue::from_str(value)?),
                "accept-language" => headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(value)?),
                "cache-control" => headers.insert(CACHE_CONTROL, HeaderValue::from_str(value)?),
                "pragma" => headers.insert(PRAGMA, HeaderValue::from_str(value)?),
                "upgrade-insecure-requests" => headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_str(value)?),
                "cookie" => headers.insert(COOKIE, HeaderValue::from_str(value)?),
                _ => None,
            };
        }
    }

    let client = reqwest::Client::new();
    let response = client.get(url)
        .headers(headers)
        .send()
        .await?;

    let content = response.text().await?;
    Ok(content)
}
