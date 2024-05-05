// qwfy/rust/video_watch/src/common/file_common.rs

use chrono::TimeZone;
use std::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;


fn check_if_named_by_creation_time(file_path: &Path) -> Result<bool, Box<dyn Error>> {
    let metadata = fs::metadata(file_path)?;
    let systime = metadata.created()?;
    let datetime = systime.duration_since(UNIX_EPOCH)?;
    let local_datetime = chrono::Local
        .timestamp_opt(datetime.as_secs() as i64, datetime.subsec_nanos())
        .unwrap();
    let formatted_creation_time = local_datetime.format("%H-%M-%S").to_string();

    if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
        // 假设文件名格式为 "CocoMiracle轻奢女装_2024-03-06_23-49-48_000.mp4"
        // 你需要根据实际格式调整splitn和nth的参数
        if let Some(timestamp_section) = file_name.splitn(3, '_').nth(2) {
            // 提取文件名中的时间戳部分
            let timestamp_in_file_name = timestamp_section.splitn(2, '_').next().unwrap_or("");
            // 比较
            return Ok(timestamp_in_file_name == formatted_creation_time);
        }
    }

    Ok(false)
}

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
                    println!("文件重命名 到 {:?}", new_path);
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
                    // 如果是 .mp4 文件，则检查文件名是否已按创建时间命名
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        let already_named_by_creation_time =
                            check_if_named_by_creation_time(&path)?;

                        if !already_named_by_creation_time {
                            // 如果文件名尚未按创建时间命名，则尝试重命名
                            match change_filename_based_on_creation_time(&path) {
                                Ok(new_path) => {
                                    println!(
                                        "MP4 file renamed based on creation time: {:?}",
                                        new_path
                                    );
                                }
                                Err(e) => {
                                    eprintln!("Error renaming file: {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}


// use std::error::Error;
// use std::fs;
// use std::path::Path;

pub fn copy_file_to_douyin(file_path: &str) -> Result<(), Box<dyn Error>> {
    let output_root = "/Users/qwfy/douyin-cut";
    let streamer_list = vec!["魏老板私服", "刘一一","Bella刘一一"];

    // 从 file_path 中提取目录和文件名
    let path = Path::new(file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    // 提取顶级目录名和直接父目录名
    let top_dir = path.parent().unwrap().parent().unwrap().file_name().unwrap();
    let parent_dir = path.parent().unwrap().file_name().unwrap();

    // 检查文件名是否包含主播名称
    let streamer_name = streamer_list
        .iter()
        .find(|&name| file_name.contains(name))
        .map(|&name| name)
        .unwrap_or(parent_dir.to_str().unwrap());
    // println!("主播名称：{}",streamer_name);

    // 从文件名提取日期 2024-03-25
    let parts: Vec<&str> = file_name.split('_').collect();
    let date = parts[parts.len() - 3];
    // println!("日期：{}",date);


    // 创建目标目录路径
    let dest_dir = Path::new(output_root)
        .join(top_dir)
        .join(streamer_name)
        .join(format!("{}_{}", streamer_name, date));
    // println!("目标目录: {:?}", dest_dir);

    // 提取文件名的后面部分 2024-03-25_10-57-51_000.mp4
    let afterFilename = format!("{}_{}_{}", parts[parts.len() - 3], parts[parts.len() - 2], parts[parts.len() - 1]);
    // println!("文件名的后面部分为: {}", afterFilename);

    // 创建目标目录（如果不存在）
    fs::create_dir_all(&dest_dir)?;

    // 创建目标文件名
    let dest_file_name = format!("{}_{}", streamer_name, afterFilename);
    let dest_path = dest_dir.join(dest_file_name);
    println!("{}",dest_path.display());

    fs::copy(file_path, dest_path)?;

    Ok(())
}

pub fn do_copy() {
    let source_file = "../../../downloads/抖音直播/@魏老板私服/@魏老板私服_2024-03-25_10-57-51_000.mp4";
    let source_file_2 = "../../../downloads/抖音直播/刘一一(周二早上7_30分直播)/刘一一(周二早上7_30分直播)_2024-04-23_07-48-15_019.mp4";

    match copy_file_to_douyin(source_file) {
        Ok(()) => println!("文件复制成功！"),
        Err(e) => println!("无法复制文件：{}", e),
    }

    match copy_file_to_douyin(source_file_2) {
        Ok(()) => println!("文件复制成功！"),
        Err(e) => println!("无法复制文件：{}", e),
    }
}

