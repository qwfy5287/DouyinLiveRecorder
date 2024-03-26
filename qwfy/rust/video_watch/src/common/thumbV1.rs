// qwfy/rust/video_watch/src/common/thumb.rs

use std::fs;
use std::path::Path;
use std::process::Command;

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
                if let Some(capture) = filename
                    .strip_prefix(video_filename)
                    .and_then(|f| f.strip_suffix(".jpg"))
                {
                    let frame_number_str = capture.trim_start_matches('_');

                    match frame_number_str.parse::<i32>() {
                        Ok(frame_number) => {
                            // 使用 frame_number 的逻辑
                            let frame_number = frame_number_str.parse::<i32>().unwrap();
                            // 将帧号转换为时间，这里假设每15帧为一秒
                            let seconds = frame_number * 15;
                            let new_filename =
                                format!("{}_{}.jpg", video_filename, seconds_to_timestamp(seconds));
                            let new_path = dir.join(new_filename);
                            fs::rename(path, new_path)?;
                        }
                        Err(e) => {
                            // 错误处理
                            eprintln!(
                                "Failed to parse frame number from filename: {}. Error: {}",
                                filename, e
                            );
                            continue; // 跳过当前循环迭代
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn seconds_to_timestamp(seconds: i32) -> String {
    format!(
        "{:02}_{:02}_{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}

pub fn process_videos(base_path: &str) {
    let video_ext = "mp4"; // 定义视频文件的扩展名

    match fs::read_dir(base_path) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == video_ext {
                    let video_name = path.file_stem().unwrap_or_default().to_string_lossy();
                    let input_video_path = path.to_string_lossy().into_owned();
                    let output_dir = format!("{}/{}_thumb", base_path, video_name);
                    let output_pattern =
                        format!("{}/{}_thumb/{}_%03d.jpg", base_path, video_name, video_name);

                    if let Err(e) = ensure_directory_exists(&output_dir) {
                        eprintln!(
                            "Error creating output directory for '{}': {}",
                            video_name, e
                        );
                        continue;
                    }

                    if let Err(e) = generate_thumbnails(&input_video_path, &output_pattern) {
                        eprintln!("Error generating thumbnails for '{}': {}", video_name, e);
                        continue;
                    }

                    if let Err(e) = rename_files(&output_dir, &video_name) {
                        eprintln!("Error renaming files for '{}': {}", video_name, e);
                        continue;
                    }

                    println!("Thumbnails generated successfully for '{}'", video_name);
                }
            }
        }
        Err(e) => eprintln!("Failed to read directory '{}': {}", base_path, e),
    }
}

pub fn process_video(file_path: &Path) {
    let video_name = file_path.file_stem().unwrap_or_default().to_string_lossy();
    let output_dir = format!(
        "{}/{}_thumb",
        file_path.parent().unwrap().to_string_lossy(),
        video_name
    );
    let output_pattern = format!(
        "{}/{}_thumb/{}_%03d.jpg",
        file_path.parent().unwrap().to_string_lossy(),
        video_name,
        video_name
    );

    if let Err(e) = ensure_directory_exists(&output_dir) {
        eprintln!(
            "Error creating output directory for '{}': {}",
            video_name, e
        );
        return;
    }

    if let Err(e) = generate_thumbnails(file_path.to_str().unwrap(), &output_pattern) {
        eprintln!("Error generating thumbnails for '{}': {}", video_name, e);
        return;
    }

    if let Err(e) = rename_files(&output_dir, &video_name) {
        eprintln!("Error renaming files for '{}': {}", video_name, e);
        return;
    }

    println!("Thumbnails generated successfully for '{}'", video_name);
}