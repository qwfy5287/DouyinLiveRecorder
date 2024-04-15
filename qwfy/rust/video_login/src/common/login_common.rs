// qwfy/rust/video_login/src/common/login_common.rs

use machine_uid::get;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
    pub device_id: String,
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
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let login_url = "https://124.70.131.130/api/douyin-user/login";
    let device_id = get().map_err(|e| format!("Failed to get device ID: {}", e))?;
    let login_request = LoginRequest {
        phone,
        password,
        device_id,
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

pub fn handle_login_result(result: Result<LoginResponse, Box<dyn std::error::Error>>) {
    match result {
        Ok(response) => {
            if response.code == 20000 {
                println!("登录成功: {}", response.message);
            } else {
                eprintln!("登录失败: {}", response.message);
                println!("请确认【用户名】和【密码】正确！");
                println!("如有软件问题");
                println!("请发送短信 '无法登录' 到 18250833087 获取技术支持。");
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("请求错误: {}", err);
            println!("登录失败，请确认【用户名】和【密码】正确！");
            println!("请确认【用户名】和【密码】正确！");
            println!("如有软件问题");
            println!("请发送短信 '无法登录' 到 18250833087 获取技术支持。");
            process::exit(1);
        }
    }
}