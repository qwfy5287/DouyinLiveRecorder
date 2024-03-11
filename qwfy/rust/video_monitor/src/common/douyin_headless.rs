// use headless_chrome::{Browser, Tab};
// use serde_json::Value;
// use std::{error::Error, thread, time::Duration};

// use crate::common::douyin_file::write_live_link_to_file;

// pub fn navigate_and_extract(tab: &Tab, url: &str) -> Result<(), Box<dyn Error>> {
//     tab.navigate_to(url)?.wait_until_navigated()?;

//     thread::sleep(Duration::from_secs(1));

//     // Print the page source for debugging
//     let html_content = tab.get_content()?;
//     // println!("页面HTML内容:\n{}", html_content);

//     let result = tab
//         .evaluate(&extraction_script(), true)?
//         .value
//         .ok_or("Failed to extract data")?;
//     let live_info: Value = serde_json::from_str(result.as_str().ok_or("Invalid JSON format")?)?;

//     println!("直播链接: {}", live_info["liveLink"]);
//     println!("直播标题: {}", live_info["liveTitle"]);

//     if live_info["liveLink"] != "直播链接未找到" {
//         write_live_link_to_file(live_info["liveLink"].as_str().unwrap())?;
//     }

//     Ok(())
// }

// fn extraction_script() -> String {
//     r#"
//         (() => {
//             const liveLinkElement = document.querySelector('a[href^="https://live.douyin.com/"]');
//             let liveLink = liveLinkElement ? liveLinkElement.href : '直播链接未找到';
//             if (liveLink !== '直播链接未找到') {
//                 liveLink = liveLink.split('?')[0];
//             }

//             const userInfoElement = document.querySelector('[data-e2e="user-info"]');
//             let liveTitle = userInfoElement ? userInfoElement.querySelector('h1')?.textContent || '直播标题未找到' : '直播标题未找到';

//             return JSON.stringify({ liveLink, liveTitle });
//         })()
//     "#.to_string()
// }
