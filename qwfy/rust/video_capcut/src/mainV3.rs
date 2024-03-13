use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn split_video_by_subtitles(draft_content_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 读取 draft_content.json 文件
    let data = fs::read_to_string(draft_content_path)?;
    let mut value: Value = serde_json::from_str(&data)?;

    // 获取视频轨道和字幕轨道
    let tracks = value["tracks"].as_array_mut().unwrap();

    // 获取视频片段和字幕片段
    let video_segments = tracks[0]["segments"].as_array().unwrap();
    let subtitle_segments = tracks[1]["segments"].as_array().unwrap();

    // 创建新的视频片段数组
    let mut new_video_segments = Vec::new();

    // 记录上一个字幕片段的结束时间
    let mut last_subtitle_end = 0;

    // 记录视频素材ID与新片段的映射关系
    let mut material_segments_map: HashMap<String, Vec<Value>> = HashMap::new();

    // 遍历字幕片段
    for subtitle_segment in subtitle_segments {
        let subtitle_start = subtitle_segment["target_timerange"]["start"]
            .as_u64()
            .unwrap();
        let subtitle_end = subtitle_start
            + subtitle_segment["target_timerange"]["duration"]
                .as_u64()
                .unwrap();

        // 如果当前字幕片段的起始时间大于上一个字幕片段的结束时间,
        // 则在两个字幕片段之间插入一个新的视频片段
        if subtitle_start > last_subtitle_end {
            let mut new_segment = video_segments[0].clone();
            new_segment["target_timerange"]["start"] = json!(last_subtitle_end);
            new_segment["target_timerange"]["duration"] = json!(subtitle_start - last_subtitle_end);
            new_video_segments.push(new_segment.clone());

            let material_id = new_segment["material_id"].as_str().unwrap().to_string();
            material_segments_map
                .entry(material_id)
                .or_insert(Vec::new())
                .push(new_segment);
        }

        // 插入与当前字幕片段对应的视频片段
        let mut new_segment = video_segments[0].clone();
        new_segment["target_timerange"]["start"] = json!(subtitle_start);
        new_segment["target_timerange"]["duration"] = json!(subtitle_end - subtitle_start);
        new_video_segments.push(new_segment.clone());

        let material_id = new_segment["material_id"].as_str().unwrap().to_string();
        material_segments_map
            .entry(material_id)
            .or_insert(Vec::new())
            .push(new_segment);

        // 更新上一个字幕片段的结束时间
        last_subtitle_end = subtitle_end;
    }

    // 如果最后一个字幕片段的结束时间小于整个视频的时长,
    // 则在最后插入一个新的视频片段
    let video_duration = video_segments[0]["target_timerange"]["duration"]
        .as_u64()
        .unwrap();
    if last_subtitle_end < video_duration {
        let mut new_segment = video_segments[0].clone();
        new_segment["target_timerange"]["start"] = json!(last_subtitle_end);
        new_segment["target_timerange"]["duration"] = json!(video_duration - last_subtitle_end);
        new_video_segments.push(new_segment.clone());

        let material_id = new_segment["material_id"].as_str().unwrap().to_string();
        material_segments_map
            .entry(material_id)
            .or_insert(Vec::new())
            .push(new_segment);
    }

    // 更新视频轨道的片段数组
    tracks[0]["segments"] = json!(new_video_segments);

    // 更新素材中的视频片段引用
    let materials = value["materials"].as_object_mut().unwrap();
    let videos = materials["videos"].as_array_mut().unwrap();
    for video in videos {
        let material_id = video["id"].as_str().unwrap();
        if let Some(segments) = material_segments_map.get(material_id) {
            video["segments"] = json!(segments);
        }
    }

    // 更新素材中的画布、音频等其他引用
    // ...

    // 将修改后的 JSON 写回文件
    let new_data = serde_json::to_string_pretty(&value)?;
    fs::write(draft_content_path, new_data)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");

    // ./data/draft_content.json
    let draft_content_path = "./data/draft_content.json";
    split_video_by_subtitles(draft_content_path).unwrap();

    println!("分割视频成功！");
}
