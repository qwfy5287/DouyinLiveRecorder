// qwfy/rust/video_thumb_frame/src/common/douyin_image_common.rs

use std::fs::File;
use std::io::Write;
use serde::Serialize;
use std::fs::{create_dir_all, read_dir};


#[derive(Serialize)]
pub struct DouyinImageInfo {
    id: String,
    file_name: String,
    first_folder: String,
    second_folder: String,
    third_folder: String,
    fourth_folder: String,
}

pub fn generate_douyin_image_info_file(output_dir: &str, video_path: &str) -> Result<(), String> {
    let mut thumbnail_info_vec = Vec::new();

    let entries = match read_dir(output_dir) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    };

    for (index, entry) in entries.enumerate() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                let components: Vec<&str> = video_path.split('/').collect();
                let first_folder = components[components.len() - 4].to_string();
                let second_folder = components[components.len() - 3].to_string();
                let third_folder = components[components.len() - 2].to_string();
                let fourth_folder = path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();

                let thumbnail_info = DouyinImageInfo {
                    id: format!("{:05}", index),
                    file_name,
                    first_folder,
                    second_folder,
                    third_folder,
                    fourth_folder,
                };

                thumbnail_info_vec.push(thumbnail_info);
            }
        }
    }

    let json_path = format!("{}/thumbnail_info.json", output_dir);
    let json_string = serde_json::to_string_pretty(&thumbnail_info_vec).unwrap();

    let mut file = match File::create(&json_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to create JSON file: {}", e)),
    };

    if let Err(e) = file.write_all(json_string.as_bytes()) {
        return Err(format!("Failed to write JSON file: {}", e));
    }

    Ok(())
}


pub fn generate_douyin_image_info_vec(output_dir: &str, video_path: &str) -> Result<Vec<DouyinImageInfo>, String> {
    let mut thumbnail_info_vec = Vec::new();
    let entries = match read_dir(output_dir) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    };

    for (index, entry) in entries.enumerate() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                let components: Vec<&str> = video_path.split('/').collect();
                let first_folder = components[components.len() - 4].to_string();
                let second_folder = components[components.len() - 3].to_string();
                let third_folder = components[components.len() - 2].to_string();
                let fourth_folder = path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();

                let thumbnail_info = DouyinImageInfo {
                    id: format!("{:05}", index),
                    file_name,
                    first_folder,
                    second_folder,
                    third_folder,
                    fourth_folder,
                };
                thumbnail_info_vec.push(thumbnail_info);
            }
        }
    }

    Ok(thumbnail_info_vec)
}