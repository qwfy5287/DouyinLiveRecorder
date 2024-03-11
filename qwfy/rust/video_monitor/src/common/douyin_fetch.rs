// qwfy/rust/video_monitor/src/common/douyin_fetch.rs

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, COOKIE, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS};
use std::fs;

// use serde_json::Value;
use std::{error::Error};


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


pub fn extract_from_content(content: &str) -> Result<(String, String), Box<dyn Error>> {
    let document = scraper::Html::parse_document(content);

    let live_link_selector = scraper::Selector::parse("a[href^='https://live.douyin.com/']").unwrap();
    let live_link = document
        .select(&live_link_selector)
        .next()
        .map(|element| element.value().attr("href").unwrap_or("直播链接未找到").split('?').next().unwrap_or("直播链接未找到"))
        .unwrap_or("直播链接未找到")
        .to_string();

    let live_title_selector = scraper::Selector::parse("[data-e2e='user-info'] h1").unwrap();
    let live_title = document
        .select(&live_title_selector)
        .next()
        .map(|element| element.text().collect::<String>().trim().to_string())
        .unwrap_or_else(|| "直播标题未找到".to_string());

    Ok((live_link, live_title))
}
