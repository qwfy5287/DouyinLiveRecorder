use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// 提取ffmpeg调用为独立函数，增加可读性和可复用性
fn generate_thumbnails(video_path: &Path, output_pattern: &str) -> Result<(), String> {
    Command::new("ffmpeg")
        .args([
            "-y", // 覆盖输出文件
            "-i",
            video_path.to_str().ok_or("Invalid video path")?,
            "-vf",
            "fps=1/15", // 生成频率为每15帧1张
            "-vsync",
            "vfr", // 避免帧同步问题
            output_pattern,
        ])
        .status()
        .map_err(|_| "Failed to execute ffmpeg")?
        .success()
        .then(|| ())
        .ok_or_else(|| "Failed to generate thumbnails".into())
}

// 确保输出目录存在
fn ensure_directory_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

// 重命名缩略图文件
fn rename_thumbnail_files(output_dir: &Path, video_filename: &str) -> std::io::Result<()> {
    let thumbnail_pattern = format!("{}_{}.jpg", video_filename, "{:03}");
    fs::read_dir(output_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path.file_name().unwrap_or_default().to_str() == Some(&thumbnail_pattern)
        })
        .try_for_each(|path| {
            let new_name = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|stem| stem.split('_').last())
                .map(|frame| seconds_to_timestamp(frame.parse::<i32>().unwrap() * 15))
                .map(|timestamp| format!("{}_{}.jpg", video_filename, timestamp))
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to parse filename",
                ));
            let new_path = output_dir.join(new_name?);
            fs::rename(&path, &new_path)
        })
}

// 将帧转换为时间戳
fn seconds_to_timestamp(seconds: i32) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}

// 处理视频文件，生成缩略图
fn process_videos(base_path: &Path) -> Result<(), String> {
    fs::read_dir(base_path)
        .map_err(|_| "Failed to read base directory".to_string())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "mp4")
        .try_for_each(|video_path| {
            let video_name = video_path
                .file_stem()
                .ok_or("Invalid video file")?
                .to_string_lossy();
            let output_dir = base_path.join(format!("{}_thumb", video_name));
            let output_pattern = output_dir.join(format!("{}_{}.jpg", video_name, "{:03}"));

            ensure_directory_exists(&output_dir)
                .map_err(|_| "Failed to create output directory".to_string())?;
            generate_thumbnails(
                &video_path,
                output_pattern.to_str().ok_or("Invalid output pattern")?,
            )?;
            rename_thumbnail_files(&output_dir, &video_name)
                .map_err(|_| "Failed to rename files".into())
        })
}

fn main() {
    let base_path = PathBuf::from("../../../downloads/抖音直播/诗篇女装旗舰店");
    if let Err(e) = process_videos(&base_path) {
        eprintln!("Error processing videos: {}", e);
    }
}
