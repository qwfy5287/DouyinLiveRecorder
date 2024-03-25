// qwfy/rust/video_watch/src/common/thumb.rs

use std::fs;
use std::path::Path;
use std::process::Command;

fn extract_frame(video_path: &str, output_path: &str, timestamp: u64) -> Result<(), String> {
    let output = Command::new("ffmpeg")
        .args(&[
            "-ss",
            &format!("{}", timestamp),
            "-i",
            video_path,
            "-vframes",
            "1",
            "-q:v",
            "2",
            output_path,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn ensure_directory_exists(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn get_video_duration(video_path: &str) -> Result<f64, String> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            video_path,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    let duration_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    duration_str
        .parse::<f64>()
        .map_err(|_| "Failed to parse video duration".to_string())
}

fn generate_timestamps(duration: f64, interval: f64) -> Vec<u64> {
    let num_frames = (duration / interval).ceil() as u64;
    (0..num_frames).map(|i| (i as f64 * interval) as u64).collect()
}

fn seconds_to_timestamp(seconds: u64) -> String {
    format!(
        "{:02}_{:02}_{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}

pub fn process_videos(base_path: &str) {
    let video_ext = "mp4"; // 定义视频文件的扩展名
    let interval = 30.0; // 每隔 15 秒提取一帧

    match fs::read_dir(base_path) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == video_ext {
                    let video_name = path.file_stem().unwrap_or_default().to_string_lossy();
                    let input_video_path = path.to_string_lossy().into_owned();
                    let output_dir = format!("{}/{}_thumb", base_path, video_name);

                    if let Err(e) = ensure_directory_exists(&output_dir) {
                        eprintln!(
                            "Error creating output directory for '{}': {}",
                            video_name, e
                        );
                        continue;
                    }

                    match get_video_duration(&input_video_path) {
                        Ok(duration) => {
                            let timestamps = generate_timestamps(duration, interval);
                            for &timestamp in &timestamps {
                                let output_path = format!(
                                    "{}/{}_{}.jpg",
                                    output_dir,
                                    video_name,
                                    seconds_to_timestamp(timestamp)
                                );
                                if let Err(e) =
                                    extract_frame(&input_video_path, &output_path, timestamp)
                                {
                                    eprintln!(
                                        "Error extracting frame at timestamp {} for '{}': {}",
                                        timestamp, video_name, e
                                    );
                                    continue;
                                }
                            }
                            println!("Thumbnails generated successfully for '{}'", video_name);
                        }
                        Err(e) => {
                            eprintln!("Error getting video duration for '{}': {}", video_name, e);
                            continue;
                        }
                    }
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
    let interval = 30.0; // 每隔 15 秒提取一帧

    if let Err(e) = ensure_directory_exists(&output_dir) {
        eprintln!(
            "Error creating output directory for '{}': {}",
            video_name, e
        );
        return;
    }

    match get_video_duration(file_path.to_str().unwrap()) {
        Ok(duration) => {
            let timestamps = generate_timestamps(duration, interval);
            for &timestamp in &timestamps {
                let output_path = format!(
                    "{}/{}_{}.jpg",
                    output_dir,
                    video_name,
                    seconds_to_timestamp(timestamp)
                );
                if let Err(e) = extract_frame(file_path.to_str().unwrap(), &output_path, timestamp) {
                    eprintln!(
                        "Error extracting frame at timestamp {} for '{}': {}",
                        timestamp, video_name, e
                    );
                    continue;
                }
            }
            println!("Thumbnails generated successfully for '{}'", video_name);
        }
        Err(e) => {
            eprintln!("Error getting video duration for '{}': {}", video_name, e);
            return;
        }
    }
}