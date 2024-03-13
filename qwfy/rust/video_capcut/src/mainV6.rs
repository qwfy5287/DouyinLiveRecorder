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

    // 记录字幕片段的起始时间
    let mut subtitle_start_times = Vec::new();

    // 遍历字幕片段
    for subtitle_segment in subtitle_segments {
        let subtitle_start = subtitle_segment["target_timerange"]["start"]
            .as_u64()
            .unwrap();
        let subtitle_end = subtitle_start
            + subtitle_segment["target_timerange"]["duration"]
                .as_u64()
                .unwrap();
        subtitle_start_times.push(subtitle_start);

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
    }

    // 在字幕片段之间插入视频片段
    let mut prev_subtitle_end = 0;
    for &subtitle_start in &subtitle_start_times {
        if subtitle_start > prev_subtitle_end {
            let mut new_segment = video_segments[0].clone();
            new_segment["target_timerange"]["start"] = json!(prev_subtitle_end);
            new_segment["target_timerange"]["duration"] = json!(subtitle_start - prev_subtitle_end);
            new_video_segments.insert(new_video_segments.len() - 1, new_segment.clone());

            let material_id = new_segment["material_id"].as_str().unwrap().to_string();
            material_segments_map
                .entry(material_id)
                .or_insert(Vec::new())
                .push(new_segment);
        }
        prev_subtitle_end = subtitle_start;
    }

    // 如果最后一个字幕片段的结束时间小于整个视频的时长，
    // 则在最后插入一个新的视频片段
    let video_duration = video_segments[0]["source_timerange"]["duration"]
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

    // 在视频片段数组的开头插入一个新的视频片段
    if !subtitle_start_times.is_empty() {
        let mut new_segment = video_segments[0].clone();
        new_segment["target_timerange"]["start"] = json!(0);
        new_segment["target_timerange"]["duration"] = json!(subtitle_start_times[0]);
        new_video_segments.insert(0, new_segment.clone());
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
    update_material_refs(&mut value, &new_video_segments);

    // 将修改后的 JSON 写回文件
    let new_data = serde_json::to_string_pretty(&value)?;
    fs::write(draft_content_path, new_data)?;

    Ok(())
}

fn update_material_refs(value: &mut Value, new_video_segments: &[Value]) {
    let materials = value["materials"].as_object_mut().unwrap();

    // 更新画布引用
    let canvases = materials["canvases"].as_array_mut().unwrap();
    let mut canvas_ids = Vec::new();
    for segment in new_video_segments {
        if let Some(canvas_id) = segment["extra_material_refs"][1].as_str() {
            canvas_ids.push(canvas_id);
        }
    }
    canvases.retain(|canvas| canvas_ids.contains(&canvas["id"].as_str().unwrap()));

    // 更新音频通道映射引用
    let sound_channel_mappings = materials["sound_channel_mappings"].as_array_mut().unwrap();
    let mut sound_channel_mapping_ids = Vec::new();
    for segment in new_video_segments {
        if let Some(sound_channel_mapping_id) = segment["extra_material_refs"][2].as_str() {
            sound_channel_mapping_ids.push(sound_channel_mapping_id);
        }
    }
    sound_channel_mappings
        .retain(|mapping| sound_channel_mapping_ids.contains(&mapping["id"].as_str().unwrap()));

    // 更新速度引用
    let speeds = materials["speeds"].as_array_mut().unwrap();
    let mut speed_ids = Vec::new();
    for segment in new_video_segments {
        if let Some(speed_id) = segment["extra_material_refs"][0].as_str() {
            speed_ids.push(speed_id);
        }
    }
    speeds.retain(|speed| speed_ids.contains(&speed["id"].as_str().unwrap()));

    // 更新人声分离引用
    let vocal_separations = materials["vocal_separations"].as_array_mut().unwrap();
    let mut vocal_separation_ids = Vec::new();
    for segment in new_video_segments {
        if let Some(vocal_separation_id) = segment["extra_material_refs"][3].as_str() {
            vocal_separation_ids.push(vocal_separation_id);
        }
    }
    vocal_separations
        .retain(|separation| vocal_separation_ids.contains(&separation["id"].as_str().unwrap()));
}

fn main() {
    println!("Hello, world!");

    // ./data/draft_content.json
    let draft_content_path = "./data/draft_content.json";
    split_video_by_subtitles(draft_content_path).unwrap();

    println!("分割视频成功！");
}
