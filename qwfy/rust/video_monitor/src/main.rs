// qwfy/rust/video_monitor/src/main.rs

use std::{error::Error, thread, time::Duration};

mod common{
  pub mod douyin_file;
  pub mod douyin_headless;
  pub mod douyin_fetch;
}

use crate::common::{
    douyin_file::{read_urls_from_file, write_live_link_to_file},
    douyin_headless::navigate_and_extract,
    douyin_fetch::fetch_url,
};

use headless_chrome::Browser;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut browser = Browser::default()?;
    let urls = read_urls_from_file("./config/user_list.ini")?;

    let refresh_interval = 30;
    let use_headless = false; // 设置为 true 使用 headless,设置为 false 使用 fetch

    loop {
        for url in &urls {
            if use_headless {
                let tab_result = browser.new_tab();

                match tab_result {
                    Ok(tab) => {
                        let extract_result = navigate_and_extract(&tab, url);
                        if extract_result.is_err() {
                            println!("Error navigating to URL: {}", url);
                        }
                    }
                    Err(e) => {
                        println!("无法打开新标签,重新初始化 Browser: {}", e);
                        browser = Browser::default()?;
                        let tab = browser.new_tab()?;
                        navigate_and_extract(&tab, url)?;
                    }
                }
            } else {
                
                match fetch_url(url).await {
                    Ok(content) => {
                        // println!("页面HTML内容:\n{}", content);

                        // 处理 fetch 结果
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

fn extract_from_content(content: &str) -> Result<(String, String), Box<dyn Error>> {
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
