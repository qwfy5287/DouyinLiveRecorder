// qwfy/rust/video_monitor/src/main.rs

// use std::error::Error;

// use headless_chrome::protocol::cdp::Page;
// use headless_chrome::Browser;

// fn browse_wikipedia() -> Result<(), Box<dyn Error>> {
//     let browser = Browser::default()?;

//     let tab = browser.new_tab()?;

//     /// Navigate to wikipedia
//     tab.navigate_to("https://www.wikipedia.org")?;

//     /// Wait for network/javascript/dom to make the search-box available
//     /// and click it.
//     tab.wait_for_element("input#searchInput")?.click()?;

//     /// Type in a query and press `Enter`
//     tab.type_str("WebKit")?.press_key("Enter")?;

//     /// We should end up on the WebKit-page once navigated
//     let elem = tab.wait_for_element("#firstHeading")?;
//     assert!(tab.get_url().ends_with("WebKit"));

//     /// Take a screenshot of the entire browser window
//     let _jpeg_data =
//         tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;

//     /// Take a screenshot of just the WebKit-Infobox
//     let _png_data = tab
//         .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
//         .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;

//     // Run JavaScript in the page
//     let remote_object = elem.call_js_fn(
//         r#"
//         function getIdTwice () {
//             // `this` is always the element that you called `call_js_fn` on
//             const id = this.id;
//             return id + id;
//         }
//     "#,
//         vec![],
//         false,
//     )?;
//     match remote_object.value {
//         Some(returned_string) => {
//             dbg!(&returned_string);
//             assert_eq!(returned_string, "firstHeadingfirstHeading".to_string());
//         }
//         _ => unreachable!(),
//     };

//     Ok(())
// }

// fn main() {
//     browse_wikipedia().expect("REASON")
// }

// use headless_chrome::{protocol::page::methods::CaptureScreenshotFormat, Browser};
// use std::error::Error;

// fn screenshot_baidu() -> Result<(), Box<dyn Error>> {
//     let browser = Browser::default()?;
//     let tab = browser.new_tab()?;

//     // 导航到百度首页
//     tab.navigate_to("https://www.baidu.com")?;
//     // 等待页面加载完毕，这里简单使用固定时间等待，实际应用中可能需要更精细的控制
//     std::thread::sleep(std::time::Duration::from_secs(5));

//     // 捕获屏幕截图，这里假设保存为PNG格式
//     let screenshot_data = tab.capture_screenshot(CaptureScreenshotFormat::PNG, None, None, true)?;

//     // 保存截图到文件
//     std::fs::write("baidu_screenshot.png", screenshot_data)?;

//     Ok(())
// }

// fn main() {
//     if let Err(e) = screenshot_baidu() {
//         eprintln!("Error taking screenshot of Baidu: {:?}", e);
//     } else {
//         println!("Screenshot saved as baidu_screenshot.png");
//     }
// }

// use headless_chrome::{protocol::cdp::Page, Browser};
// use std::error::Error;

// fn main() -> Result<(), Box<dyn Error>> {
//     // 创建浏览器实例
//     let browser = Browser::default()?;

//     // 创建一个新的浏览器标签页
//     let tab = browser.new_tab()?;

//     // 导航到 "https://www.baidu.com"
//     // tab.navigate_to("https://www.baidu.com")?;
//     // tab.navigate_to(
//     //     "https://www.douyin.com/search/%E9%AD%8F%E8%80%81%E6%9D%BF?source=switch_tab&type=live",
//     // )?;

//     // tab.navigate_to("https://www.douyin.com/search/65181878010?source=switch_tab&type=user")?;
//     // tab.navigate_to("https://live.douyin.com/599952912242")?;
//     tab.navigate_to("https://www.douyin.com/user/MS4wLjABAAAAF-Ne-5HXXXmdzAGhuZygBQUpDTK3IbEHJfFYPAhVfRfyihhVB2sz0vYO0aofyGnP")?;

//     // 等待页面加载完成
//     tab.wait_until_navigated()?;

//     // 等待页面加载完毕，这里简单使用固定时间等待，实际应用中可能需要更精细的控制
//     std::thread::sleep(std::time::Duration::from_secs(5));

//     // 捕获整个页面的屏幕截图
//     let screenshot_data = tab.capture_screenshot(
//         Page::CaptureScreenshotFormatOption::Png, // 截图格式为 PNG
//         None,                                     // 使用默认视图宽度
//         None,                                     // 使用默认视图高度
//         true,                                     // 从设备的像素比例中捕获
//     )?;

//     // 保存屏幕截图到文件
//     std::fs::write("dy.png", screenshot_data)?;

//     println!("Screenshot saved as 'dy.png'");

//     Ok(())
// }
//
//

use headless_chrome::{protocol::cdp::Page, Browser};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 创建浏览器实例
    let browser = Browser::default()?;

    // 创建一个新的浏览器标签页
    let tab = browser.new_tab()?;

    // 导航到 "https://www.baidu.com"
    // tab.navigate_to("https://www.baidu.com")?;
    // tab.navigate_to(
    //     "https://www.douyin.com/search/%E9%AD%8F%E8%80%81%E6%9D%BF?source=switch_tab&type=live",
    // )?;

    // tab.navigate_to("https://www.douyin.com/search/65181878010?source=switch_tab&type=user")?;
    // tab.navigate_to("https://live.douyin.com/599952912242")?;
    tab.navigate_to("https://www.douyin.com/user/MS4wLjABAAAAF-Ne-5HXXXmdzAGhuZygBQUpDTK3IbEHJfFYPAhVfRfyihhVB2sz0vYO0aofyGnP")?;

    // 等待页面加载完成
    tab.wait_until_navigated()?;

    // 等待页面加载完毕，这里简单使用固定时间等待，实际应用中可能需要更精细的控制
    std::thread::sleep(std::time::Duration::from_secs(5));

    // 捕获整个页面的屏幕截图
    let screenshot_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Png, // 截图格式为 PNG
        None,                                     // 使用默认视图宽度
        None,                                     // 使用默认视图高度
        true,                                     // 从设备的像素比例中捕获
    )?;

    // 保存屏幕截图到文件
    std::fs::write("dy.png", screenshot_data)?;

    println!("Screenshot saved as 'dy.png'");

    Ok(())
}
