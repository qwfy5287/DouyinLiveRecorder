use std::fs;
use std::io::Error;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
struct JsonData {
    tracks: Vec<Track>,
    materials: Materials,
}

#[derive(Serialize, Deserialize, Clone)]
struct Track {
    #[serde(rename = "type")]
    track_type: String,
    segments: Vec<Segment>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Segment {
    material_id: String,
    target_timerange: TargetTimerange,
}

#[derive(Serialize, Deserialize, Clone)]
struct TargetTimerange {
    start: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Materials {
    texts: Vec<Text>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Text {
    id: String,
    words: Words,
}

#[derive(Serialize, Deserialize, Clone)]
struct Words {
    text: Vec<String>,
    end_time: Vec<u64>,
}


// 从文件中读取 JSON 数据
fn read_json_from_file(file_path: &str) -> Result<JsonData, Error> {
    let json_string = fs::read_to_string(file_path)?;
    let json_data: JsonData = serde_json::from_str(&json_string)?;
    Ok(json_data)
}

// 将时间从微秒转换为毫秒
fn convert_time_to_milliseconds(time: u64) -> u64 {
    time / 1000
}

fn json_to_srt(json: &JsonData, keyword_list: &[&str]) -> String {
    let mut srt = Vec::new();
    let mut index = 1;

    // 预处理 tracks 中的时间单位
    let mut tracks = json.tracks.clone();
    for track in &mut tracks {
        if track.track_type == "text" {
            for segment in &mut track.segments {
                segment.target_timerange.start = convert_time_to_milliseconds(segment.target_timerange.start);
            }
        }
    }

    // 遍历每个文本轨道
    for track in &tracks {
        if track.track_type == "text" {
            // 遍历轨道中的每个片段
            for segment in &track.segments {
                let material = json.materials.texts.iter().find(|text| text.id == segment.material_id).unwrap();

                // 构建字幕内容
                let mut subtitle_lines = Vec::new();
                let mut current_line = SubtitleLine {
                    text: String::new(),
                    start_time: segment.target_timerange.start,
                    end_time: segment.target_timerange.start,
                };
                for (i, word) in material.words.text.iter().enumerate() {
                    let word_end_time = material.words.end_time[i] + segment.target_timerange.start;

                    if word_end_time - current_line.end_time <= 500 {
                        current_line.text.push_str(word);
                        current_line.end_time = word_end_time;
                    } else {
                        subtitle_lines.push(current_line.clone());
                        current_line = SubtitleLine {
                            text: word.to_string(),
                            start_time: current_line.end_time,
                            end_time: word_end_time,
                        };
                    }
                }
                subtitle_lines.push(current_line);

                // 根据关键词列表分割字幕内容
                let mut split_subtitle_lines = Vec::new();
                for line in subtitle_lines {
                    let mut start_index = 0;
                    for keyword in keyword_list {
                        if let Some(keyword_index) = line.text[start_index..].find(keyword) {
                            let keyword_index = start_index + keyword_index;
                            if keyword_index > start_index {
                                split_subtitle_lines.push(SubtitleLine {
                                    text: line.text[start_index..keyword_index].to_string(),
                                    start_time: line.start_time,
                                    end_time: line.start_time + ((line.end_time - line.start_time) * (keyword_index - start_index) as u64 / line.text.len() as u64) as u64,
                                });
                            }
                            split_subtitle_lines.push(SubtitleLine {
                                text: keyword.to_string(),
                                start_time: line.start_time + ((line.end_time - line.start_time) * keyword_index as u64 / line.text.len() as u64) as u64,
                                end_time: line.start_time + ((line.end_time - line.start_time) * (keyword_index + keyword.len()) as u64 / line.text.len() as u64) as u64,
                            });
                            start_index = keyword_index + keyword.len();
                        }
                    }
                    if start_index < line.text.len() {
                        split_subtitle_lines.push(SubtitleLine {
                            text: line.text[start_index..].to_string(),
                            start_time: line.start_time + ((line.end_time - line.start_time) * start_index as u64 / line.text.len() as u64) as u64,
                            end_time: line.end_time,
                        });
                    }
                }

                // 将分割后的字幕内容转换为 SRT 格式
                for line in split_subtitle_lines {
                    let start_time = format_time(line.start_time);
                    let end_time = format_time(line.end_time);

                    srt.push(index.to_string());
                    srt.push(format!("{} --> {}", start_time, end_time));
                    srt.push(line.text);
                    srt.push(String::new());

                    index += 1;
                }
            }
        }
    }

    srt.join("\n")
}

// 格式化时间为 SRT 格式
fn format_time(time: u64) -> String {
    let total_seconds = time / 1000;
    let milliseconds = time % 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, milliseconds)
}

#[derive(Clone)]
struct SubtitleLine {
    text: String,
    start_time: u64,
    end_time: u64,
}

fn main() {
    // 读取 JSON 文件
    let json_data = read_json_from_file("./draft_info_example.json");

    if let Ok(json_data) = json_data {
        let keyword_list = vec!["买的", "黑色的", "好不好", "然后呢", "呃"];
        let srt_data = json_to_srt(&json_data, &keyword_list);
        println!("{}", srt_data);
    } else {
        eprintln!("读取 JSON 文件时出错");
    }
}