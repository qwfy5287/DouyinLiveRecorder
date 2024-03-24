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
    words: Words,
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

    let keywords = vec!["现货".to_string(), "一个".to_string()];

    let mut srt_content = String::new();
    let mut subtitle_index = 1;
    let track = &draft_info.tracks[0];

    for segment in &track.segments {
        let material = material_map.get(&segment.material_id).unwrap();
        let segment_start_time = segment.target_timerange.start;
        let segment_end_time = segment_start_time + segment.target_timerange.duration;

        let mut subtitle_text = extract_text(&material.content);
        let mut filtered_words = Vec::new();

        for (index, word) in material.words.text.iter().enumerate() {
            if keywords.contains(word) {
                filtered_words.push((
                    material.words.start_time[index],
                    material.words.end_time[index],
                    word,
                ));
            }
        }

        let mut remaining_text = subtitle_text.clone();
        let mut last_end_time = segment_start_time;

        for (start, end, word) in &filtered_words {
            let start_ms = segment_start_time + start;
            let end_ms = segment_start_time + end;

            if start_ms > last_end_time {
                srt_content.push_str(&format!("{}\n", subtitle_index));
                srt_content.push_str(&format_time_range(last_end_time, start_ms));
                srt_content.push_str(&format!(
                    "{}\n\n",
                    &remaining_text[..remaining_text
                        .find(word.as_str())
                        .unwrap_or(remaining_text.len())]
                ));
                subtitle_index += 1;
            }

            remaining_text = remaining_text.replacen(word.as_str(), "", 1);
            last_end_time = end_ms;
        }

        if last_end_time < segment_end_time {
            srt_content.push_str(&format!("{}\n", subtitle_index));
            srt_content.push_str(&format_time_range(last_end_time, segment_end_time));
            srt_content.push_str(&format!("{}\n\n", remaining_text.trim()));
            subtitle_index += 1;
        }
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
    let ms = timestamp % 1000;
    let seconds = (timestamp / 1000) % 60;
    let minutes = (timestamp / 60000) % 60;
    let hours = timestamp / 3600000;

    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, ms)
}

fn extract_text(content: &str) -> String {
    content
        .trim_matches(|c| c == '{' || c == '}')
        .trim_start_matches(r#""text":""#)
        .trim_end_matches(r#"""#)
        .to_string()
}
