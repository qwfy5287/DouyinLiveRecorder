use std::process::Command;

fn split_video(file_path: &str, segment_time: u32) -> std::io::Result<()> {
    let output_prefix = "output_segment_";
    Command::new("ffmpeg")
        .args([
            "-i",
            file_path,
            "-c",
            "copy",
            "-map",
            "0",
            "-segment_time",
            &format!("{}", segment_time),
            "-f",
            "segment",
            "-reset_timestamps",
            "1",
            &format!("{}%03d.mp4", output_prefix),
        ])
        .status()?;

    Ok(())
}

fn main() {
    match split_video("input.mp4", 20) {
        Ok(()) => println!("Video successfully split."),
        Err(e) => eprintln!("Failed to split video: {}", e),
    }
}
