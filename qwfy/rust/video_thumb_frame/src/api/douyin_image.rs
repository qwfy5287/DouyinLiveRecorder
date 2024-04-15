// qwfy/rust/video_thumb_frame/src/api/douyin_image.rs

// use serde::Serialize;

// VITE_API_BASE_URL=https://124.70.131.130/api
const BASE_URL: &str = "https://124.70.131.130/api";


use crate::common::douyin_image_common::DouyinImageInfo;


/**
 * 创建抖音图片json
 * @param {Object} data data
 */
pub async fn douyin_image_create(data: DouyinImageInfo) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    println!("{:?}", data);
    let url = format!("{}/douyin-image/create", BASE_URL);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data)?)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}