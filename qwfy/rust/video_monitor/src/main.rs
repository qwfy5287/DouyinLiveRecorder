// qwfy/rust/video_monitor/src/main.rs

mod common;

use std::{error::Error, thread, time::Duration};

use crate::common::{
    douyin_file::{read_urls_from_file, write_live_link_to_file},
    douyin_headless::navigate_and_extract,
    douyin_fetch::{fetch_url, extract_from_content},
};

use headless_chrome::Browser;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let urls = read_urls_from_file("./config/user_list.ini")?;

    let refresh_interval = 30;
    let use_headless = false; // 设置为 true 使用 headless,设置为 false 使用 fetch

    let mut browser = None;
    if use_headless {
        browser = Some(Browser::default()?);
    }

    loop {
        for url in &urls {
            if use_headless {
                if let Some(ref mut b) = browser {
                    let tab_result = b.new_tab();
                    match tab_result {
                        Ok(tab) => {
                            let extract_result = navigate_and_extract(&tab, url);
                            if extract_result.is_err() {
                                println!("Error navigating to URL: {}", url);
                            }
                        }
                        Err(e) => {
                            println!("无法打开新标签,重新初始化 Browser: {}", e);
                            *b = Browser::default()?;
                            let tab = b.new_tab()?;
                            navigate_and_extract(&tab, url)?;
                        }
                    }
                }
            } else {
                match fetch_url(url).await {
                    Ok(content) => {
                        match extract_from_content(&content) {
                            Ok((live_link, live_title)) => {
                                println!("直播链接: {}", live_link);
                                println!("直播标题: {}", live_title);
                                if live_link != "直播链接未找到" {
                                    if let Err(e) = write_live_link_to_file(&live_link) {
                                        println!("写入直播链接到文件时出错: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("提取直播信息时出错: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error fetching URL: {}", e);
                    }
                }
            }
        }
        println!("等待{}秒后继续处理下一轮URLs", refresh_interval);
        thread::sleep(Duration::from_secs(refresh_interval));
    }
}
