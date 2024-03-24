use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
struct DraftInfo {
    materials: Materials,
    tracks: Vec<Track>,
}

#[derive(Deserialize)]
struct Materials {
    texts: Vec<Text>,
}

#[derive(Deserialize)]
struct Text {
    id: String,
    content: String,
}

#[derive(Deserialize)]
struct Words {
    start_time: Vec<u64>,
    end_time: Vec<u64>,
    text: Vec<String>,
}

#[derive(Deserialize)]
struct Track {
    segments: Vec<Segment>,
}

#[derive(Deserialize)]
struct Segment {
    material_id: String,
    target_timerange: TargetTimeRange,
}

#[derive(Deserialize)]
struct TargetTimeRange {
    start: u64,
    duration: u64,
}

fn main() {
    let mut file = File::open("./draft_info_example.json").expect("无法打开文件");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("无法读取文件");

    let draft_info: DraftInfo = serde_json::from_str(&json_data).expect("无法解析 JSON");

    let mut material_map = HashMap::new();
    for text in &draft_info.materials.texts {
        material_map.insert(&text.id, text);
    }

    let mut srt_content = String::new();
    let track = &draft_info.tracks[0];

    for (index, segment) in track.segments.iter().enumerate() {
        let material = material_map.get(&segment.material_id).unwrap();
        let start_time = segment.target_timerange.start;
        let end_time = start_time + segment.target_timerange.duration;

        srt_content.push_str(&format!("{}\n", index + 1));
        srt_content.push_str(&format_time_range(start_time, end_time));
        srt_content.push_str(&format!("{}\n\n", extract_text(&material.content)));
    }

    println!("{}", srt_content);
}

fn format_time_range(start: u64, end: u64) -> String {
    format!(
        "{} --> {}\n",
        format_timestamp(start),
        format_timestamp(end)
    )
}

fn format_timestamp(timestamp: u64) -> String {
    let ms = timestamp / 1000;
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    format!(
        "{:02}:{:02}:{:02},{:03}",
        hours,
        minutes % 60,
        seconds % 60,
        ms % 1000
    )
}

fn extract_text(content: &str) -> &str {
    content
        .trim_matches(|c| c == '{' || c == '}')
        .trim_start_matches(r#""text":""#)
        .trim_end_matches(r#"""#)
}
