
use std::process::Command;
use std::fs::{create_dir_all, read_dir};
use std::path::Path;
use rayon::prelude::*;

use tokio::time::sleep;
use std::time::Duration;

use crate::common::douyin_image_common::generate_douyin_image_info_vec;
use crate::common::sync_common::{sync_single_folder};
use crate::api::douyin_image::douyin_image_create;
use async_recursion::async_recursion;


pub fn get_video_duration(video_path: &str) -> Result<f64, String> {
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

fn seconds_to_timestamp(seconds: u64) -> String {
    format!(
        "{:02}_{:02}_{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
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

async fn process_video(video_path: &str, output_dir: &str, interval: f64) {
    if !Path::new(output_dir).exists() {
        match get_video_duration(video_path) {
            Ok(duration) => {
                let timestamps = generate_timestamps(duration, interval);
                match extract_frames(video_path, output_dir, &timestamps) {
                    Ok(_) =>{
                        println!("Frames extracted successfully for: {}", video_path);
                        println!("output_dir: {}", output_dir);

                        // 1. 上传文件
                        sync_single_folder(output_dir);
                        
                        // 2. 调用 api
                        if let Ok(thumbnail_info_vec) = generate_douyin_image_info_vec(output_dir, video_path) {
                            for thumbnail_info in thumbnail_info_vec {
                                if let Err(e) = douyin_image_create(thumbnail_info).await {
                                    eprintln!("Error creating douyin image JSON: {}", e);
                                }
                                sleep(Duration::from_millis(10)).await; // 延迟 10 毫秒
                            }
                        } else {
                            eprintln!("Error generating thumbnail info");
                        }

                    },
                    Err(e) => eprintln!("Error processing {}: {}", video_path, e),
                }
            }
            Err(e) => eprintln!("Error processing {}: {}", video_path, e),
        }
    } else {
        println!("Skipping {:?} as output directory already exists", output_dir);
    }
}

#[async_recursion]
pub async fn process_directory(dir_path: &str, input_root: &str, output_root: &str, interval: f64) {
    if let Ok(entries) = read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "mp4" || ext == "avi" || ext == "mov" {
                            let video_path = path.to_str().unwrap();
                            let relative_path = path.strip_prefix(input_root).unwrap();
                            let output_dir = format!("{}/{}_thumb", output_root, relative_path.with_extension("").to_str().unwrap());
                            process_video(video_path, &output_dir, interval).await;
                        }
                    }
                } else if path.is_dir() {
                    let subdir_path = path.to_str().unwrap();
                    process_directory(subdir_path, input_root, output_root, interval).await;
                }
            }
        }
    }
}


pub async fn do_process_directory() {

    let input_root ="/Users/qwfy/douyin-cut".to_string();

    let output_root ="/Users/qwfy/douyin-thumb".to_string();

    let interval = 30.0; // 每隔 30 秒提取一帧

    process_directory(&input_root, &input_root, &output_root, interval).await;

}