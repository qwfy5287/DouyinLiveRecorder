use std::process::Command;

fn extract_frames(video_path: &str, output_dir: &str, timestamps: &[u64]) -> Result<(), String> {
    for (i, &timestamp) in timestamps.iter().enumerate() {
        let output_path = format!("{}/frame_{}.jpg", output_dir, i + 1);
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
    let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
    let output_dir = "output";

    let timestamps = &[30, 80, 200, 300];

    match extract_frames(video_path, output_dir, timestamps) {
        Ok(_) => println!("Frames extracted successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}