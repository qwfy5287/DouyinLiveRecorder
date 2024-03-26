
// ./common/login_common.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub code: u32,
    pub message: String,
    pub count: u32,
    pub records: Vec<serde_json::Value>,
}

pub async fn login(phone: String, password: String) -> Result<LoginResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let login_url = "http://localhost:20248/douyin-user/login";
    
    let login_request = LoginRequest {
        phone,
        password,
        
    };
    
    let response = client
        .post(login_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&login_request)?)
        .send()
        .await?;
        
    let login_response: LoginResponse = response.json().await?;
    
    Ok(login_response)
}