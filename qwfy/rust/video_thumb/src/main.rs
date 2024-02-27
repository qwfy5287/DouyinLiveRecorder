// qwfy/rust/video_thumb/src/main.rs

// qwfy/rust/video_thumb/data/@魏老板私服_2024-02-26_07-14-44_025.mp4

use std::process::Command;

use std::fs;
use std::path::{Path, PathBuf};

fn generate_thumbnails(file_path: &str, output_pattern: &str) -> Result<(), String> {
    let status = Command::new("ffmpeg")
        .arg("-y") // 自动确认覆盖
        .arg("-i")
        .arg(file_path)
        .arg("-vf")
        .arg("fps=1/15") // 每秒 15 帧
        .arg("-vsync")
        .arg("vfr") // 避免重复帧
        .arg(output_pattern) // 输出文件模式，例如: output_%03d.jpg
        .status()
        .expect("Failed to execute ffmpeg");

    match status.success() {
        true => Ok(()),
        false => Err("Failed to generate thumbnails".into()),
    }
}


fn ensure_directory_exists(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}


fn rename_files(input_dir: &str, video_filename: &str) -> std::io::Result<()> {
    let dir = Path::new(input_dir);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap();
                // 假设文件名遵循 "output_001.jpg" 的格式
                if let Some(capture) = filename.strip_prefix("output_").and_then(|f| f.strip_suffix(".jpg")) {
                    let frame_number = capture.parse::<i32>().unwrap();
                    // 将帧号转换为时间，这里假设每15帧为一秒
                    let seconds = frame_number * 15;
                    let new_filename = format!("{}_{}.jpg", video_filename, seconds_to_timestamp(seconds));
                    let new_path = dir.join(new_filename);
                    fs::rename(path, new_path)?;
                }
            }
        }
    }
    Ok(())
}

// 将秒转换为 "00:00:10" 格式的时间戳
fn seconds_to_timestamp(seconds: i32) -> String {
    format!("{:02}.{:02}.{:02}", seconds / 3600, (seconds % 3600) / 60, seconds % 60)
}


fn main() {
    let input_video_path = "./data/@魏老板私服_2024-02-26_07-14-44_025.mp4";
    let output_pattern = "./data/@魏老板私服_2024-02-26_07-14-44_025/thumbnails/output_%03d.jpg";

    let output_dir = "./data/@魏老板私服_2024-02-26_07-14-44_025/thumbnails";
    match ensure_directory_exists(output_dir) {
        Ok(_) => println!("Output directory is ready."),
        Err(e) => eprintln!("Error creating output directory: {}", e),
    }

    match generate_thumbnails(input_video_path, output_pattern) {
        Ok(_) => println!("Thumbnails generated successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }

    let input_dir = "./data/@魏老板私服_2024-02-26_07-14-44_025/thumbnails";
    let video_filename = "@魏老板私服_2024-02-26_07-14-44_025";
    match rename_files(input_dir, video_filename) {
        Ok(_) => println!("Files renamed successfully"),
        Err(e) => eprintln!("Error renaming files: {}", e),
    }
}