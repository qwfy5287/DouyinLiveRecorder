// use serde::Deserialize;
// use std::fs;
// use std::process::Command;

// #[derive(Deserialize)]
// struct Segment {
//     keyword: String,
//     text: String,
//     start_time: String,
//     end_time: String,
// }

// fn split_video_segments(file_path: &str, segments_json: &str) -> std::io::Result<()> {
//     let segments: Vec<Segment> = serde_json::from_str(segments_json).unwrap();

//     for (index, segment) in segments.iter().enumerate() {
//         let output_file = format!(
//             "segment_{}_{}.mp4",
//             index,
//             segment.keyword.replace(" ", "_")
//         );

//         let start_time = segment.start_time.replace(",", ".");
//         let end_time = segment.end_time.replace(",", ".");

//         Command::new("ffmpeg")
//             .args([
//                 "-i",
//                 file_path,
//                 "-ss",
//                 &start_time,
//                 "-to",
//                 &end_time,
//                 "-c:v",
//                 "libx264", // 指定视频编码器
//                 "-c:a",
//                 "aac", // 指定音频编码器
//                 &output_file,
//             ])
//             .status()?;
//         println!("Segment saved: {}", output_file);
//     }

//     Ok(())
// }

// fn main() {
//     let file_path = "./data/example.mp4";
//     let segments_json = fs::read_to_string("./data/example.json").expect("Unable to read file");

//     match split_video_segments(file_path, &segments_json) {
//         Ok(()) => println!("Video successfully split."),
//         Err(e) => eprintln!("Failed to split video: {}", e),
//     }
// }

use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
struct Segment {
    keyword: String,
    text: String,
    start_time: String,
    end_time: String,
}

// 从 .srt 文件中提取段落
fn extract_segments_from_srt(srt_content: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let lines: Vec<&str> = srt_content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].parse::<i32>().is_ok() {
            let times: Vec<&str> = lines[i + 1].split(" --> ").collect();
            let start_time = times[0].replace(",", ".");
            let end_time = times[1].replace(",", ".");
            let mut text = String::new();
            i += 2;
            while i < lines.len() && !lines[i].is_empty() {
                text.push_str(lines[i]);
                text.push(' ');
                i += 1;
            }
            segments.push(Segment {
                keyword: format!("Segment {}", segments.len() + 1),
                text,
                start_time,
                end_time,
            });
        }
        i += 1;
    }

    segments
}

fn split_video_segments(file_path: &str, segments: Vec<Segment>) -> std::io::Result<()> {
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
                "libx264",
                "-c:a",
                "aac",
                &output_file,
            ])
            .status()?;
        println!("Segment saved: {}", output_file);
    }

    Ok(())
}

fn process_json_file(file_path: &str, segments_file_path: &str) -> std::io::Result<()> {
    let segments_json = fs::read_to_string(segments_file_path).expect("Unable to read file");
    let segments: Vec<Segment> = serde_json::from_str(&segments_json).unwrap();
    split_video_segments(file_path, segments)
}

fn main() {
    let file_path = "./data/example.mp4";
    // let segments_file_path = "./data/example.srt"; // 假设这里是.srt文件路径
    let segments_file_path = "./data/example.json"; // 假设这里是.srt文件路径

    // if Path::new(segments_file_path).extension().unwrap() == "srt" {
    //     let srt_content = fs::read_to_string(segments_file_path).expect("Unable to read .srt file");
    //     let segments = extract_segments_from_srt(&srt_content);
    //     split_video_segments(file_path, segments)
    //         .expect("Failed to split video based on .srt file");
    // } else {
    //     // 如果不是 .srt 文件，则按照原有逻辑处理，例如处理 .json 文件
    //     println!("Unsupported file format.");
    // }
    match Path::new(&segments_file_path)
        .extension()
        .and_then(|s| s.to_str())
    {
        Some("json") => {
            process_json_file(file_path, &segments_file_path)
                .expect("Failed to split video based on .json file");
        }
        Some("srt") => {
            let srt_content =
                fs::read_to_string(&segments_file_path).expect("Unable to read .srt file");
            let segments = extract_segments_from_srt(&srt_content);
            split_video_segments(file_path, segments)
                .expect("Failed to split video based on .srt file");
        }
        _ => println!("Unsupported file format."),
    }
}
