use headless_chrome::{Browser, Tab};
use serde_json::Value;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut browser = Browser::default()?;
    let urls = read_urls_from_file("./config/user_list.ini")?;

    // 刷新时间间隔
    let refresh_interval = 30;

    loop {
        for url in &urls {
            let tab_result = browser.new_tab();

            match tab_result {
                Ok(tab) => {
                    let extract_result = navigate_and_extract(&tab, url);
                    if extract_result.is_err() {
                        // Handle error, log it, or decide to break/continue
                        println!("Error navigating to URL: {}", url);
                    }
                    // Ensure to close or release the tab resource here if applicable
                }
                Err(e) => {
                    println!("无法打开新标签，重新初始化 Browser: {}", e);
                    // Consider re-initializing the browser if necessary
                    browser = Browser::default()?;
                    let tab = browser.new_tab()?;
                    navigate_and_extract(&tab, url)?;
                }
            }
        }

        println!("等待{}秒后继续处理下一轮URLs", refresh_interval);
        thread::sleep(Duration::from_secs(refresh_interval));
    }
}

// // 循环
// fn main() -> Result<(), Box<dyn Error>> {
//     let browser = Browser::default()?;
//     let urls = read_urls_from_file("./config/user_list.ini")?;

//     loop {
//         // 对每个URL执行操作
//         for url in &urls {
//             let tab = browser.new_tab()?;
//             navigate_and_extract(&tab, url)?;
//         }

//         println!("等待20秒后继续处理下一轮URLs");
//         // 完成一轮处理所有URLs后，等待20秒
//         thread::sleep(Duration::from_secs(20));
//     }
// }

// 单次
// fn main() -> Result<(), Box<dyn Error>> {
//     let browser = Browser::default()?;

//     let urls = read_urls_from_file("./config/user_list.ini")?;

//     for url in urls {
//         let tab = browser.new_tab()?;
//         navigate_and_extract(&tab, &url)?;
//     }

//     Ok(())
// }

fn navigate_and_extract(tab: &Tab, url: &str) -> Result<(), Box<dyn Error>> {
    tab.navigate_to(url)?.wait_until_navigated()?;

    thread::sleep(Duration::from_secs(1)); // Consider using more reliable wait conditions if possible

    let result = tab
        .evaluate(&extraction_script(), true)?
        .value
        .ok_or("Failed to extract data")?;
    let live_info: Value = serde_json::from_str(result.as_str().ok_or("Invalid JSON format")?)?;

    println!("直播链接: {}", live_info["liveLink"]);
    println!("直播标题: {}", live_info["liveTitle"]);

    // If liveLink is found and not the placeholder text, write to file
    if live_info["liveLink"] != "直播链接未找到" {
        write_live_link_to_file(live_info["liveLink"].as_str().unwrap())?;
    }

    Ok(())
}

fn extraction_script() -> String {
    r#"
        (() => {
            const liveLinkElement = document.querySelector('a[href^="https://live.douyin.com/"]');
            let liveLink = liveLinkElement ? liveLinkElement.href : '直播链接未找到';
            if (liveLink !== '直播链接未找到') {
                liveLink = liveLink.split('?')[0];
            }

            const userInfoElement = document.querySelector('[data-e2e="user-info"]');
            let liveTitle = userInfoElement ? userInfoElement.querySelector('h1')?.textContent || '直播标题未找到' : '直播标题未找到';

            return JSON.stringify({ liveLink, liveTitle });
        })()
    "#.to_string()
}

fn write_live_link_to_file(live_link: &str) -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("../../../config");
    fs::create_dir_all(&config_path)?;

    let file_path = config_path.join("URL_config.ini");
    let mut content = String::new();

    // Read the existing content if the file exists
    if file_path.exists() {
        content = fs::read_to_string(&file_path)?;
    }

    // Check if the live_link is already present to avoid duplicates
    if !content.contains(live_link) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;

        // Write the live_link if it's not found in the content, manage initial empty line
        if !content.is_empty() {
            writeln!(file, "{}", live_link)?;
        } else {
            write!(file, "{}", live_link)?;
        }
    }

    Ok(())
}

fn read_urls_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let buf = io::BufReader::new(file);
    let urls = buf
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.split(',').next().unwrap().trim().to_string())
        .collect();
    Ok(urls)
}
