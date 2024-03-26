use std::process::Command;
use std::fs::{create_dir_all, read_dir};
use std::path::Path;
use rayon::prelude::*;

fn seconds_to_timestamp(seconds: u64) -> String {
    format!(
        "{:02}_{:02}_{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
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
    let mut timestamps = Vec::new();
    let mut current_time = 0.0;

    while current_time < duration {
        timestamps.push(current_time as u64);
        current_time += interval;
    }

    timestamps
}

fn extract_frame(video_path: &str, output_dir: &str, timestamp: u64) -> Result<(), String> {
    let video_file_name = Path::new(video_path).file_stem().unwrap().to_str().unwrap();
    let output_path = format!("{}/{}_{}.jpg", output_dir, video_file_name, seconds_to_timestamp(timestamp));

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
            &output_path,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn extract_frames(video_path: &str, output_dir: &str, timestamps: &[u64]) -> Result<(), String> {
    create_dir_all(&output_dir).expect("Failed to create subdirectory");

    let results: Vec<Result<(), String>> = timestamps
        .par_iter()
        .map(|&timestamp| extract_frame(video_path, output_dir, timestamp))
        .collect();

    if results.iter().all(|r| r.is_ok()) {
        Ok(())
    } else {
        Err("Failed to extract some frames".to_string())
    }
}

fn process_video(video_path: &str, output_dir: &str, interval: f64) {
    match get_video_duration(video_path) {
        Ok(duration) => {
            let timestamps = generate_timestamps(duration, interval);
            match extract_frames(video_path, output_dir, &timestamps) {
                Ok(_) => println!("Frames extracted successfully for: {}", video_path),
                Err(e) => eprintln!("Error processing {}: {}", video_path, e),
            }
        }
        Err(e) => eprintln!("Error processing {}: {}", video_path, e),
    }
}

fn process_directory(dir_path: &str, output_dir: &str, interval: f64) {
    if let Ok(entries) = read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "mp4" || ext == "avi" || ext == "mov" {
                            let video_path = path.to_str().unwrap();
                            let video_file_name = path.file_stem().unwrap().to_str().unwrap();
                            let video_output_dir = format!("{}/{}_thumb", output_dir, video_file_name);
                            process_video(video_path, &video_output_dir, interval);
                        }
                    }
                } else if path.is_dir() {
                    let subdir_path = path.to_str().unwrap();
                    let subdir_output_dir = format!("{}/{}", output_dir, path.file_name().unwrap().to_str().unwrap());
                    process_directory(subdir_path, &subdir_output_dir, interval);
                }
            }
        }
    }
}

fn main() {
    // let path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18/@魏老板私服_2024-03-18_07-33-23.mp4";
    // let path = "../../../../../douyinCut/魏老板现货号_2024-03-23";
    // let output_dir = "../../../../../douyinCut/魏老板现货号_2024-03-23_output";
  
    // let path = "../../../../../douyin-cut/刘11生活号_2024-03-23";
    // let output_dir = "../../../../../douyin-thumb/抖音直播/刘11生活号/刘11生活号_2024-03-23_thumb";
  

    let path = "/Volumes/qwfy-wd-2t/抖音直播/@魏老板私服_2024-03-25";
    let output_dir = "../../../../../douyin-thumb/抖音直播/@魏老板私服/@魏老板私服_2024-03-25_thumb";
  
  
    
    // let output_dir = "output";
    let interval = 30.0; // 每隔 60 秒提取一帧

    if Path::new(path).is_file() {
        process_video(path, output_dir, interval);
    } else if Path::new(path).is_dir() {
        process_directory(path, output_dir, interval);
    } else {
        eprintln!("Invalid path: {}", path);
    }
}