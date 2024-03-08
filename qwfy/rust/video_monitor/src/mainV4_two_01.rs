// qwfy/rust/video_monitor/src/main.rs

use std::{error::Error, thread, time::Duration};

mod common{
  pub mod douyin_file;
  pub mod douyin_headless;
  pub mod douyin_fetch;
}

use crate::common::{
    douyin_file::read_urls_from_file,
    douyin_headless::navigate_and_extract,
    douyin_fetch::fetch_url,
};

use headless_chrome::Browser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut browser = Browser::default()?;
    let urls = read_urls_from_file("./config/user_list.ini")?;

    let refresh_interval = 30;
    let use_headless = true; // 设置为 true 使用 headless,设置为 false 使用 fetch

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
                        println!("页面HTML内容:\n{}", content);
                        // 在这里添加处理 fetch 结果的代码
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
