// qwfy/rust/video_thumb/src/main.rs

// qwfy/rust/video_thumb/data/@魏老板私服_2024-02-26_07-14-44_025.mp4

// 引入必要的库
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
                // 直接处理 "@魏老板私服_2024-02-26_07-14-44_025_001.jpg" 格式的文件名
                if let Some(capture) = filename.strip_prefix(video_filename).and_then(|f| f.strip_suffix(".jpg")) {
                    let frame_number_str = capture.trim_start_matches('_');
                    let frame_number = frame_number_str.parse::<i32>().unwrap();
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

fn seconds_to_timestamp(seconds: i32) -> String {
    format!("{:02}_{:02}_{:02}", seconds / 3600, (seconds % 3600) / 60, seconds % 60)
}

fn main() {
    let base_path = "./data";
    let video_name = "@魏老板私服_2024-02-26_07-14-44_025";
    let input_video_path = format!("{}/{}.mp4", base_path, video_name);
    let output_dir = format!("{}/{}_thumb", base_path, video_name);
    let output_pattern = format!("{}/{}_thumb/{}_%03d.jpg", base_path, video_name, video_name);

    if let Err(e) = ensure_directory_exists(&output_dir) {
        eprintln!("Error creating output directory: {}", e);
        return;
    }

    if let Err(e) = generate_thumbnails(&input_video_path, &output_pattern) {
        eprintln!("Error generating thumbnails: {}", e);
        return;
    }

    if let Err(e) = rename_files(&output_dir, video_name) {
        eprintln!("Error renaming files: {}", e);
        return;
    }

    println!("All operations completed successfully");
}
