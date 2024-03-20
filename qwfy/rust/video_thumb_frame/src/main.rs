

use std::process::Command;
use std::fs::create_dir_all;
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









// // fn extract_frame(video_path: &str, output_dir: &str, timestamp: u64) -> Result<(), String> {
// //     let output_path = format!("{}/frame_{:03}.jpg", output_dir, timestamp);
// //     let output = Command::new("ffmpeg")
// //         .args(&[
// //             "-ss",
// //             &format!("{}", timestamp),
// //             "-i",
// //             video_path,
// //             "-vframes",
// //             "1",
// //             "-q:v",
// //             "2",
// //             &output_path,
// //         ])
// //         .output()
// //         .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

// //     if output.status.success() {
// //         Ok(())
// //     } else {
// //         Err(String::from_utf8_lossy(&output.stderr).to_string())
// //     }
// // }

// fn extract_frame(video_path: &str, output_dir: &str, timestamp: u64) -> Result<(), String> {
//     let output_path = format!("{}/frame_{}_{}.jpg", output_dir, timestamp, seconds_to_timestamp(timestamp));
//     let output = Command::new("ffmpeg")
//         .args(&[
//             "-ss",
//             &format!("{}", timestamp),
//             "-i",
//             video_path,
//             "-vframes",
//             "1",
//             "-q:v",
//             "2",
//             &output_path,
//         ])
//         .output()
//         .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

//     if output.status.success() {
//         Ok(())
//     } else {
//         Err(String::from_utf8_lossy(&output.stderr).to_string())
//     }
// }

// fn extract_frames(video_path: &str, output_dir: &str, timestamps: &[u64]) -> Result<(), String> {
//     create_dir_all(&output_dir).expect("Failed to create subdirectory");

//     let results: Vec<Result<(), String>> = timestamps
//         .par_iter()
//         .map(|&timestamp| extract_frame(video_path, output_dir, timestamp))
//         .collect();

//     if results.iter().all(|r| r.is_ok()) {
//         Ok(())
//     } else {
//         Err("Failed to extract some frames".to_string())
//     }
// }

fn extract_frame(video_path: &str, output_dir: &str, timestamp: u64, index: usize) -> Result<(), String> {
    // let output_path = format!("{}/frame_{:03}_{}_{}.jpg", output_dir, index, timestamp, seconds_to_timestamp(timestamp));
    let output_path = format!("{}/frame_{:03}_{}.jpg", output_dir, index, seconds_to_timestamp(timestamp));
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
        .enumerate()
        .map(|(i, &timestamp)| extract_frame(video_path, output_dir, timestamp, i))
        .collect();

    if results.iter().all(|r| r.is_ok()) {
        Ok(())
    } else {
        Err("Failed to extract some frames".to_string())
    }
}


fn main() {
    let video_path = "../../../../../douyinCut/2024-03-10-07-39-19.mp4";
    
    let output_dir = "output";
    let interval = 60.0; // 每隔 60 秒提取一幀

    match get_video_duration(video_path) {
        Ok(duration) => {
            let timestamps = generate_timestamps(duration, interval);
            match extract_frames(video_path, output_dir, &timestamps) {
                Ok(_) => println!("Frames extracted successfully"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
