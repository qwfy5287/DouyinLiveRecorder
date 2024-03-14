use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
struct Segment {
    keyword: String,
    text: String,
    start_time: String,
    end_time: String,
}

fn split_video_segments(file_path: &str, segments_json: &str) -> std::io::Result<()> {
    let segments: Vec<Segment> = serde_json::from_str(segments_json).unwrap();

    for (index, segment) in segments.iter().enumerate() {
        let output_file = format!(
            "segment_{}_{}.mp4",
            index,
            segment.keyword.replace(" ", "_")
        );

        let start_time = segment.start_time.replace(",", ".");
        let end_time = segment.end_time.replace(",", ".");

        Command::new("ffmpeg")
            .args([
                "-i",
                file_path,
                "-ss",
                &start_time,
                "-to",
                &end_time,
                "-c:v",
                "libx264", // 指定视频编码器
                "-c:a",
                "aac", // 指定音频编码器
                &output_file,
            ])
            .status()?;
        println!("Segment saved: {}", output_file);
    }

    Ok(())
}

fn main() {
    let file_path = "./data/example.mp4";
    let segments_json = fs::read_to_string("./data/example.json").expect("Unable to read file");

    match split_video_segments(file_path, &segments_json) {
        Ok(()) => println!("Video successfully split."),
        Err(e) => eprintln!("Failed to split video: {}", e),
    }
}
