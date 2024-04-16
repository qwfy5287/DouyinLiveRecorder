// qwfy/rust/video_thumb_frame/src/main.rs

mod common;
mod api;

use video_login::common::login_common::{login, handle_login_result};

use crate::common::video_common::process_directory;


#[tokio::main]
async fn main() {
    let login_result = login("18250833087".to_string(), "qwfy@123!456".to_string()).await;
    handle_login_result(login_result);

    let args: Vec<String> = std::env::args().collect();

    let input_root = if args.len() > 1 {
        args[1].clone()
    } else {
        "/Users/qwfy/douyin-cut".to_string()
    };

    let output_root = if args.len() > 2 {
        args[2].clone()
    } else {
        "/Users/qwfy/douyin-thumb".to_string()
    };

    let interval = 30.0; // 每隔 30 秒提取一帧


    process_directory(&input_root, &input_root, &output_root, interval).await;

}