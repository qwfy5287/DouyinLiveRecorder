// qwfy/rust/video_monitor/src/common/douyin_fetch.rs

// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, COOKIE, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS};

// pub async fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let mut headers = HeaderMap::new();
//     headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"));
//     headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
//     headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,zh-TW;q=0.7,ja;q=0.6"));
//     headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
//     headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
//     headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
//     headers.insert(COOKIE, HeaderValue::from_static("douyin.com; ttwid=1%7CcQzm1t4NhUgmmnoaO0MDobLGVWS5u7ZRVIpv7IRgYzg%7C1708924897%7C87d87217eeed2873c827299b60973aa37dba5e6275aae5b61c72142105a356e3; "));

//     let client = reqwest::Client::new();
//     let response = client.get(url)
//         .headers(headers)
//         .send()
//         .await?;

//     let content = response.text().await?;
//     Ok(content)
// }

// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, COOKIE, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS};
// use std::fs;

// pub async fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let config = fs::read_to_string("./config/douyin_config.ini")?;
//     let mut headers = HeaderMap::new();

//     for line in config.lines() {
//         if let Some((key, value)) = line.split_once('=') {
//             let key = key.trim();
//             let value = value.trim();

//             match key {
//                 "user-agent" => headers.insert(USER_AGENT, HeaderValue::from_str(value)?),
//                 "accept" => headers.insert(ACCEPT, HeaderValue::from_str(value)?),
//                 "accept-language" => headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(value)?),
//                 "cache-control" => headers.insert(CACHE_CONTROL, HeaderValue::from_str(value)?),
//                 "pragma" => headers.insert(PRAGMA, HeaderValue::from_str(value)?),
//                 "upgrade-insecure-requests" => headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_str(value)?),
//                 "cookie" => headers.insert(COOKIE, HeaderValue::from_str(value)?),
//                 _ => (),
//             }
//         }
//     }

//     let client = reqwest::Client::new();
//     let response = client.get(url)
//         .headers(headers)
//         .send()
//         .await?;

//     let content = response.text().await?;
//     Ok(content)
// }

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
