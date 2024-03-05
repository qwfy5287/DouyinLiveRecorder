// qwfy/rust/video_watch/src/common/file_common.rs

use chrono::TimeZone;
use std::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use crate::common::thumb::process_video;

pub fn change_filename_based_on_creation_time(
    file_path: &PathBuf,
) -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(metadata) = fs::metadata(&file_path) {
        if let Ok(systime) = metadata.created() {
            let datetime = systime.duration_since(UNIX_EPOCH)?;
            let local_datetime = chrono::Local
                .timestamp_opt(datetime.as_secs() as i64, datetime.subsec_nanos())
                .unwrap();
            let new_time_stamp = local_datetime.format("%H-%M-%S").to_string();

            if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
                let new_file_name = {
                    let parts: Vec<&str> = file_name.rsplitn(2, '.').collect();
                    if parts.len() == 2 {
                        // 拆分以获取除时间戳外的文件名部分和扩展名
                        let name_parts: Vec<&str> = parts[1].rsplitn(4, '_').collect();
                        if name_parts.len() == 4 {
                            // 重新组合文件名，替换时间戳
                            format!(
                                "{}_{}_{}_{}.{}",
                                name_parts[3],
                                name_parts[2],
                                new_time_stamp,
                                name_parts[0],
                                parts[0]
                            )
                        } else {
                            // 如果不符合预期格式，保持原样
                            file_name.to_string()
                        }
                    } else {
                        // 如果没有扩展名，理论上不会发生，因为我们处理的是mp4文件
                        file_name.to_string()
                    }
                };

                let new_path = file_path.with_file_name(new_file_name);
                if new_path != *file_path {
                    fs::rename(file_path, &new_path)?;
                    println!("File renamed to {:?}", new_path);
                    return Ok(new_path);
                }
            }
        }
    }
    // 如果没有更改，返回原始路径
    Ok(file_path.clone())
}

/// 重命名目录下现有的所有文件
pub fn rename_existing_files(path: &Path) -> Result<(), Box<dyn Error>> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                // 如果是目录，则递归调用
                rename_existing_files(&path)?;
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("mp4") {
                    // 如果是 .mp4 文件，则尝试重命名
                    // change_filename_based_on_creation_time(&path)?;
                    match change_filename_based_on_creation_time(&path) {
                        Ok(new_path) => {
                            println!("MP4 file renamed based on creation time: {:?}", new_path);
                            // 文件可能已被重命名，`new_path` 是当前文件的路径
                            // process_video(&new_path);
                        }
                        Err(e) => {
                            eprintln!("Error renaming file: {:?}", e);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
