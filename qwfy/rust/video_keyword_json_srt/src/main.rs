extern crate rayon;
use rayon::prelude::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::env;

#[derive(Deserialize)]
struct Segment {
    keyword: String,
    text: String,
    start_time: String,
    end_time: String,
}

trait SegmentExtractor {
    fn extract_segments(&self) -> Vec<Segment>;
}

struct SrtSegmentExtractor {
    srt_content: String,
}

impl SegmentExtractor for SrtSegmentExtractor {
    fn extract_segments(&self) -> Vec<Segment> {
        let mut segments = Vec::new();
        let lines: Vec<&str> = self.srt_content.lines().collect();
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
                    keyword: format!("Segment {:02}", segments.len() + 1),
                    text,
                    start_time,
                    end_time,
                });
            }
            i += 1;
        }

        segments
    }
}

struct JsonSegmentExtractor {
    segments_json: String,
}

impl SegmentExtractor for JsonSegmentExtractor {
    fn extract_segments(&self) -> Vec<Segment> {
        serde_json::from_str(&self.segments_json).unwrap()
    }
}

struct VideoSplitter;

impl VideoSplitter {
    fn split_video_segments(
        &self,
        file_path: &str,
        segments_file_path: &str,
        segments: Vec<Segment>,
    ) -> std::io::Result<()> {
        let extension = Path::new(segments_file_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        segments
            .par_iter()
            .enumerate()
            .try_for_each(|(index, segment)| {
                let filename = Path::new(file_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap();

                let dir_path = Path::new(file_path).parent().unwrap();
                println!("dir_path: {:?}", dir_path);

                let output_dir = dir_path.join(filename).join(extension);

                std::fs::create_dir_all(&output_dir)?;

                let output_file = output_dir.join(format!(
                    "{}_segment_{:02}_{}.mp4",
                    filename,
                    index + 1,
                    segment.keyword.replace(" ", "_")
                ));

                println!("Segment text: {}", segment.text);

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
                        output_file.to_str().unwrap(),
                    ])
                    .status()?;

                println!("Segment saved: {}", output_file.display());
                Ok::<(), std::io::Error>(())
            })?;

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("使用方式: {} <file_name> [file_type]", args[0]);
        return;
    }

    let file_name = &args[1];
    let file_type = if args.len() > 2 {
        &args[2]
    } else {
        "json"
    };

    let file_path = format!("../../../../../Movies/{}.mp4", file_name);
    let segments_file_path = format!("../../../../../Movies/{}_keyword.{}", file_name, file_type);

    match file_type {
        "json" => {
            let segments_json = fs::read_to_string(&segments_file_path).expect("Unable to read file");
            let json_extractor = JsonSegmentExtractor { segments_json };
            let segments = json_extractor.extract_segments();
            let video_splitter = VideoSplitter;
            video_splitter
                .split_video_segments(&file_path, &segments_file_path, segments)
                .expect("Failed to split video based on .json file");
        }
        "srt" => {
            let srt_content = fs::read_to_string(&segments_file_path).expect("Unable to read .srt file");
            let srt_extractor = SrtSegmentExtractor { srt_content };
            let segments = srt_extractor.extract_segments();
            let video_splitter = VideoSplitter;
            video_splitter
                .split_video_segments(&file_path, &segments_file_path, segments)
                .expect("Failed to split video based on .srt file");
        }
        _ => {
            println!("Unsupported keyword file type. Supported types are 'json' and 'srt'.");
            return;
        }
    }
}