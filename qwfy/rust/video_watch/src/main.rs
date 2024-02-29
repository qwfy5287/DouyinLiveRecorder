// qwfy/rust/video_watch/src/main.rs

use notify::{recommended_watcher, RecursiveMode, Watcher, Result as NotifyResult, Event, EventKind};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::fs::{self, metadata};

use std::time::{UNIX_EPOCH};

use chrono::TimeZone;

use std::error::Error;


mod lib {
    pub mod thumb;
}

use crate::lib::thumb::process_videos;
use crate::lib::thumb::process_video;

// v1
// fn change_filename_based_on_creation_time(file_path: &PathBuf) {
//     if let Ok(metadata) = metadata(&file_path) {
//         if let Ok(systime) = metadata.created() {
//             let datetime = systime.duration_since(UNIX_EPOCH).expect("Time went backwards");
//             let local_datetime = chrono::Local.timestamp_opt(datetime.as_secs() as i64, datetime.subsec_nanos()).unwrap();
//             // 使用新的时间格式
//             let new_time_stamp = local_datetime.format("%H-%M-%S").to_string();

//             if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
//                 let new_file_name = {
//                     let parts: Vec<&str> = file_name.rsplitn(2, '.').collect();
//                     if parts.len() == 2 {
//                         // 拆分以获取除时间戳外的文件名部分和扩展名
//                         let name_parts: Vec<&str> = parts[1].rsplitn(4, '_').collect();
//                         if name_parts.len() == 4 {
//                             // 重新组合文件名，替换时间戳
//                             format!("{}_{}_{}_{}.{}", name_parts[3], name_parts[2], new_time_stamp, name_parts[0], parts[0])
//                         } else {
//                             // 如果不符合预期格式，保持原样
//                             file_name.to_string()
//                         }
//                     } else {
//                         // 如果没有扩展名，理论上不会发生，因为我们处理的是mp4文件
//                         file_name.to_string()
//                     }
//                 };

//                 let new_path = file_path.with_file_name(new_file_name);
//                 if new_path != *file_path {
//                     fs::rename(file_path, new_path.clone()).expect("Could not rename file");
//                     println!("File renamed to {:?}", new_path);
//                 }
//             }
//         }
//     }
// }


fn change_filename_based_on_creation_time(file_path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(metadata) = fs::metadata(&file_path) {
        if let Ok(systime) = metadata.created() {
            let datetime = systime.duration_since(UNIX_EPOCH)?;
            let local_datetime = chrono::Local.timestamp_opt(datetime.as_secs() as i64, datetime.subsec_nanos()).unwrap();
            let new_time_stamp = local_datetime.format("%H-%M-%S").to_string();

            if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
                let new_file_name = {
                    let parts: Vec<&str> = file_name.rsplitn(2, '.').collect();
                    if parts.len() == 2 {
                        // 拆分以获取除时间戳外的文件名部分和扩展名
                        let name_parts: Vec<&str> = parts[1].rsplitn(4, '_').collect();
                        if name_parts.len() == 4 {
                            // 重新组合文件名，替换时间戳
                            format!("{}_{}_{}_{}.{}", name_parts[3], name_parts[2], new_time_stamp, name_parts[0], parts[0])
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

// fn change_filename_based_on_creation_time(file_path: &PathBuf) -> PathBuf {
//     if let Ok(metadata) = metadata(&file_path) {
//         if let Ok(systime) = metadata.created() {
//             let datetime = systime.duration_since(UNIX_EPOCH).expect("Time went backwards");
//             let local_datetime = chrono::Local.timestamp_opt(datetime.as_secs() as i64, datetime.subsec_nanos()).unwrap();
//             let new_time_stamp = local_datetime.format("%Y-%m-%d_%H-%M-%S").to_string();

//             if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
//                 let new_file_name = format!("{}_{}.mp4", file_name, new_time_stamp);

//                 let new_path = file_path.with_file_name(new_file_name);
//                 if new_path != *file_path {
//                     fs::rename(file_path, &new_path).expect("Could not rename file");
//                     println!("File renamed to {:?}", new_path);
//                     return new_path;
//                 }
//             }
//         }
//     }
//     file_path.clone()
// }


// 新增函数用于重命名目录下现有的所有文件
fn rename_existing_files(path: &Path) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() { // 确保是文件
                change_filename_based_on_creation_time(&path);
            }
        }
    }
}

fn watch_directory(path: &str) -> NotifyResult<()> {
    let (tx, rx) = channel::<NotifyResult<Event>>();

    // 在监听之前重命名现有文件
    rename_existing_files(Path::new(path));
    // 生成缩略图
    process_videos(path);

    let mut watcher = recommended_watcher(move |res: NotifyResult<Event>| {
        match res {
            Ok(event) => {
                println!("{:?}", event);
                // 直接检查事件类型，不需要匹配携带的数据
                if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) && 
                   event.paths.iter().any(|path| path.extension().map_or(false, |ext| ext == "mp4")) {
                    if let Some(path) = event.paths.get(0) {
                        // change_filename_based_on_creation_time(path);
                        // // 现在为该文件生成缩略图
                        // process_video(path);

                        // let new_path = change_filename_based_on_creation_time(path);
                        // // 现在使用新路径为该文件生成缩略图
                        // process_video(&new_path);

                        match change_filename_based_on_creation_time(&path) {
                            Ok(new_path) => {
                                // 文件可能已被重命名，`new_path` 是当前文件的路径
                                process_video(&new_path); // 确保 `process_video` 接受 `&PathBuf` 或类似的参数
                            },
                            Err(e) => eprintln!("Error changing filename: {}", e),
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    // 事件循环
    for event in rx {
        println!("{:?}", event);
    }

    Ok(())
}


fn main() -> NotifyResult<()> {
    let path = "../../../downloads/抖音直播/Echo伊可儿女装";
    watch_directory(path)
}



// v1
// fn watch_directory(path: &str) -> NotifyResult<()> {
//     let (tx, rx) = channel::<NotifyResult<Event>>();

//     // 在监听之前重命名现有文件
//     rename_existing_files(Path::new(path));

//     // 生成缩略图
//     process_videos(path);

//     let mut watcher = recommended_watcher(move |res: NotifyResult<Event>| {
//         match res {
//             Ok(event) => {
//                 println!("{:?}", event);
//                 if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
//                     if let Some(path) = event.paths.get(0) {
//                         change_filename_based_on_creation_time(path);
//                     }
//                 }
//             }
//             Err(e) => println!("watch error: {:?}", e),
//         }
//     })?;

//     watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

//     for event in rx {
//         println!("{:?}", event);
//     }

//     Ok(())
// }