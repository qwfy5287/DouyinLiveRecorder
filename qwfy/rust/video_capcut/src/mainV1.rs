// use serde_json::json;
// use serde_json::Value;
// use std::fs;

// fn split_video_by_subtitles(draft_content_path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     // 读取 draft_content.json 文件
//     let data = fs::read_to_string(draft_content_path)?;
//     let mut value: Value = serde_json::from_str(&data)?;

//     // 获取视频轨道和字幕轨道
//     let tracks = value["tracks"].as_array_mut().unwrap();

//     // 获取视频片段和字幕片段
//     let video_segments = tracks[0]["segments"].as_array_mut().unwrap();
//     let subtitle_segments = tracks[1]["segments"].as_array().unwrap();

//     // 创建新的视频片段数组
//     let mut new_video_segments = Vec::new();

//     // 遍历字幕片段
//     for subtitle_segment in subtitle_segments {
//         let subtitle_start = subtitle_segment["target_timerange"]["start"]
//             .as_u64()
//             .unwrap();
//         let subtitle_end = subtitle_start
//             + subtitle_segment["target_timerange"]["duration"]
//                 .as_u64()
//                 .unwrap();

//         // 遍历视频片段
//         for video_segment in video_segments.iter() {
//             let video_start = video_segment["target_timerange"]["start"].as_u64().unwrap();
//             let video_end = video_start
//                 + video_segment["target_timerange"]["duration"]
//                     .as_u64()
//                     .unwrap();

//             // 判断字幕片段是否在视频片段时间范围内
//             if subtitle_start >= video_start && subtitle_end <= video_end {
//                 // 分割视频片段
//                 if subtitle_start > video_start {
//                     let mut new_segment = video_segment.clone();
//                     new_segment["target_timerange"]["duration"] =
//                         json!(subtitle_start - video_start);
//                     new_video_segments.push(new_segment);
//                 }

//                 let mut new_segment = video_segment.clone();
//                 new_segment["target_timerange"]["start"] = json!(subtitle_start);
//                 new_segment["target_timerange"]["duration"] = json!(subtitle_end - subtitle_start);
//                 new_video_segments.push(new_segment);

//                 if subtitle_end < video_end {
//                     let mut new_segment = video_segment.clone();
//                     new_segment["target_timerange"]["start"] = json!(subtitle_end);
//                     new_segment["target_timerange"]["duration"] = json!(video_end - subtitle_end);
//                     new_video_segments.push(new_segment);
//                 }
//             } else {
//                 new_video_segments.push(video_segment.clone());
//             }
//         }
//     }

//     // 更新视频轨道的片段数组
//     *video_segments = new_video_segments.into();

//     // 将修改后的 JSON 写回文件
//     let new_data = serde_json::to_string_pretty(&value)?;
//     fs::write(draft_content_path, new_data)?;

//     Ok(())
// }

use serde_json::json;
use serde_json::Value;
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

    // 遍历字幕片段
    for subtitle_segment in subtitle_segments {
        let subtitle_start = subtitle_segment["target_timerange"]["start"]
            .as_u64()
            .unwrap();
        let subtitle_end = subtitle_start
            + subtitle_segment["target_timerange"]["duration"]
                .as_u64()
                .unwrap();

        // 遍历视频片段
        for video_segment in video_segments {
            let video_start = video_segment["target_timerange"]["start"].as_u64().unwrap();
            let video_end = video_start
                + video_segment["target_timerange"]["duration"]
                    .as_u64()
                    .unwrap();

            // 判断字幕片段是否在视频片段时间范围内
            if subtitle_start >= video_start && subtitle_end <= video_end {
                // 分割视频片段
                if subtitle_start > video_start {
                    let mut new_segment = video_segment.clone();
                    new_segment["target_timerange"]["duration"] =
                        json!(subtitle_start - video_start);
                    new_video_segments.push(new_segment);
                }

                let mut new_segment = video_segment.clone();
                new_segment["target_timerange"]["start"] = json!(subtitle_start);
                new_segment["target_timerange"]["duration"] = json!(subtitle_end - subtitle_start);
                new_video_segments.push(new_segment);

                if subtitle_end < video_end {
                    let mut new_segment = video_segment.clone();
                    new_segment["target_timerange"]["start"] = json!(subtitle_end);
                    new_segment["target_timerange"]["duration"] = json!(video_end - subtitle_end);
                    new_video_segments.push(new_segment);
                }
            } else {
                new_video_segments.push(video_segment.clone());
            }
        }
    }

    // 更新视频轨道的片段数组
    tracks[0]["segments"] = json!(new_video_segments);

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
