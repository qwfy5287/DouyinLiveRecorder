use regex::Regex;
use std::process::Command;

fn parse_srt(srt_content: &str) -> Vec<(String, String)> {
    let re = Regex::new(r"(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})").unwrap();
    re.captures_iter(srt_content)
        .map(|cap| {
            let start = cap[1].replace(",", ".");
            let end = cap[2].replace(",", ".");
            (start, end)
        })
        .collect()
}

fn calculate_duration(start: &str, end: &str) -> String {
    let start_parts: Vec<&str> = start.split(':').collect();
    let end_parts: Vec<&str> = end.split(':').collect();

    let start_seconds = start_parts[0].parse::<f64>().unwrap() * 3600.0
        + start_parts[1].parse::<f64>().unwrap() * 60.0
        + start_parts[2].parse::<f64>().unwrap();

    let end_seconds = end_parts[0].parse::<f64>().unwrap() * 3600.0
        + end_parts[1].parse::<f64>().unwrap() * 60.0
        + end_parts[2].parse::<f64>().unwrap();

    format!("{:.3}", end_seconds - start_seconds)
}

fn split_video(input: &str, output_prefix: &str, timestamps: &[(String, String)]) {
    let mut i = 0;
    for (start, end) in timestamps {
        let output = format!("{}{}.mp4", output_prefix, i);
        let duration = calculate_duration(start, end);
        let status = Command::new("ffmpeg")
            .args(&[
                "-ss", start, "-i", input, "-t", &duration, "-c", "copy", &output,
            ])
            .status()
            .expect("ffmpeg命令执行失败");

        if !status.success() {
            eprintln!("ffmpeg命令执行失败, 返回码: {}", status);
            std::process::exit(1);
        }

        i += 1;
    }
}

use std::fs;

fn main() {
    let srt_file = "./subtitles.srt";
    let video_file = "./data/input.mp4";
    let output_prefix = "output_";

    let srt_content = fs::read_to_string(srt_file).expect("无法读取srt文件");
    let timestamps = parse_srt(&srt_content);
    split_video(video_file, output_prefix, &timestamps);
}
