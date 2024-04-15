// qwfy/rust/video_api/src/api/douyin_image.rs

use crate::utils::request;
use serde::Serialize;

// VITE_API_BASE_URL=https://124.70.131.130/api
const BASE_URL: &str = "https://124.70.131.130/api";

#[derive(Debug, Serialize)]
pub struct DouyinImageCreateData {
    id: String,
    file_name: String,
    first_folder: String,
    second_folder: String,
    third_folder: String,
    fourth_folder: String,
}

/**
 * 创建抖音图片json
 * @param {Object} data data
 */
pub async fn douyin_image_create(data: DouyinImageCreateData) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
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