// qwfy/rust/video_thumb/src/main.rs

use std::fs;
use std::path::{Path, PathBuf};
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
        .map_err(|_| "Failed to execute ffmpeg".to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to generate thumbnails".into())
    }
}

fn ensure_directory_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn rename_files(input_dir: &Path, video_filename: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if let Some(frame_number_str) = filename
                    .strip_prefix(video_filename)
                    .and_then(|f| f.strip_suffix(".jpg"))
                    .and_then(|f| f.trim_start_matches('_').parse::<i32>().ok())
                {
                    let seconds = frame_number_str * 15;
                    let new_filename =
                        format!("{}_{}.jpg", video_filename, seconds_to_timestamp(seconds));
                    let new_path = input_dir.join(new_filename);
                    fs::rename(path, new_path)?;
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

fn process_video(video_path: &Path, base_path: &Path) -> std::io::Result<()> {
    let video_name = video_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let output_dir = base_path.join(format!("{}_thumb", video_name));
    let output_pattern = output_dir.join(format!("{}_%03d.jpg", video_name));

    ensure_directory_exists(&output_dir)?;

    if let Err(e) = generate_thumbnails(
        video_path.to_str().unwrap(),
        output_pattern.to_str().unwrap(),
    ) {
        eprintln!("Error generating thumbnails for '{}': {}", video_name, e);
        return Ok(());
    }

    if let Err(e) = rename_files(&output_dir, video_name) {
        eprintln!("Error renaming files for '{}': {}", video_name, e);
        return Ok(());
    }

    println!("Thumbnails generated successfully for '{}'", video_name);
    Ok(())
}

fn process_videos(base_path: &Path) -> std::io::Result<()> {
    let video_ext = "mp4";
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|s| s == video_ext).unwrap_or(false) {
            process_video(&path, base_path)?;
        }
    }
    Ok(())
}

fn main() {
    let base_path = "../../../downloads/抖音直播/诗篇女装旗舰店";
    if let Err(e) = process_videos(Path::new(base_path)) {
        eprintln!("Failed to process videos in '{}': {}", base_path, e);
    }
}
