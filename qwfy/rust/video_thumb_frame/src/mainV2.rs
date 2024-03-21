
use std::process::Command;
use std::fs::create_dir_all;

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

fn extract_frames(video_path: &str, output_dir: &str, timestamps: &[u64]) -> Result<(), String> {
    for (i, &timestamp) in timestamps.iter().enumerate() {
    create_dir_all(&output_dir).expect("Failed to create subdirectory");

        let output_path = format!("{}/frame_{:03}.jpg", output_dir, i + 1);
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

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
    }

    Ok(())
}

fn main() {
    // let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
    let video_path = "../../../../../douyinCut/2024-03-10-07-39-19.mp4";
    
    let output_dir = "output";
    let interval = 60.0; // 每隔 5 秒提取一幀

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