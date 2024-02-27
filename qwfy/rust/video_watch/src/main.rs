// /Users/qwfy/GitWork/DouyinLiveRecorder/qwfy/rust/video_watch/src/main.rs


// downloads folder
// /Users/qwfy/GitWork/DouyinLiveRecorder/downloads/抖音直播/GAGAZHANG女装


use notify::{recommended_watcher, RecursiveMode, Watcher, Result as NotifyResult, Event, EventKind};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::fs::{self, metadata, ReadDir};

use std::time::{UNIX_EPOCH, SystemTime};

use chrono::TimeZone;

// fn change_filename_based_on_creation_time(file_path: &PathBuf) {
//     if let Ok(metadata) = metadata(&file_path) {
//         if let Ok(systime) = metadata.created() {
//             let datetime = systime.duration_since(UNIX_EPOCH).expect("Time went backwards");
//             // 使用 chrono 创建本地时间
//             let local_datetime = chrono::Local.timestamp(datetime.as_secs() as i64, datetime.subsec_nanos());
//             let new_timestamp = local_datetime.format("%H-%M-%S").to_string();

//             if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
//                 let new_file_name = {
//                     let parts: Vec<&str> = file_name.rsplitn(2, '.').collect();
//                     if parts.len() == 2 {
//                         // 检查并处理现有时间戳
//                         let name_without_ext = parts[1];
//                         let extension = parts[0];
//                         let new_name_without_ext = if name_without_ext.contains('_') {
//                             let name_parts: Vec<&str> = name_without_ext.rsplitn(2, '_').collect();
//                             format!("{}_{}", name_parts[1], new_timestamp)
//                         } else {
//                             format!("{}_{}", name_without_ext, new_timestamp)
//                         };
//                         format!("{}.{}", new_name_without_ext, extension)
//                     } else {
//                         // 如果没有扩展名，只添加或更新时间戳
//                         format!("{}_{}", file_name, new_timestamp)
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

// fn change_filename_based_on_creation_time(file_path: &PathBuf) {
//     if let Ok(metadata) = metadata(&file_path) {
//         if let Ok(systime) = metadata.created() {
//             let datetime = systime.duration_since(UNIX_EPOCH).expect("Time went backwards");
//             let local_datetime = chrono::Local.timestamp(datetime.as_secs() as i64, datetime.subsec_nanos());
//             let new_timestamp = local_datetime.format("%Y-%m-%d_%H-%M-%S").to_string(); // 更新时间格式

//             if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
//                 let new_file_name = {
//                     let parts: Vec<&str> = file_name.splitn(2, '.').collect();
//                     if parts.len() == 2 {
//                         let name_parts: Vec<&str> = parts[0].split('_').collect();
//                         // 仅保留到品牌名称，忽略原有时间戳
//                         let name_without_timestamp = name_parts[..name_parts.len() - 2].join("_");
//                         format!("{}_{}.{}", name_without_timestamp, new_timestamp, parts[1])
//                     } else {
//                         format!("{}_{}", file_name, new_timestamp) // 如果没有扩展名
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

// fn change_filename_based_on_creation_time(file_path: &PathBuf) {
//     if let Ok(metadata) = metadata(&file_path) {
//         if let Ok(systime) = metadata.created() {
//             let datetime = systime.duration_since(UNIX_EPOCH).expect("Time went backwards");
//             let local_datetime = chrono::Local.timestamp(datetime.as_secs() as i64, datetime.subsec_nanos());
//             // 此处的格式化字符串已调整，仅包含时间部分
//             let new_timestamp = local_datetime.format("%H-%M-%S").to_string(); 

//             if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
//                 let new_file_name = {
//                     let parts: Vec<&str> = file_name.splitn(2, '.').collect();
//                     if parts.len() == 2 {
//                         let name_parts: Vec<&str> = parts[0].split('_').collect();
//                         // 假设品牌名后即为日期，因此只取到品牌名加日期，忽略之后的时间戳
//                         let name_without_timestamp = name_parts[..name_parts.len() - 2].join("_");
//                         // 重新组合时，不再添加日期，只添加时间戳
//                         format!("{}_{}.{}", name_without_timestamp, new_timestamp, parts[1])
//                     } else {
//                         // 如果没有扩展名的情况下，理应不会出现，因为文件都应有扩展名
//                         format!("{}_{}", file_name, new_timestamp)
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

fn change_filename_based_on_creation_time(file_path: &PathBuf) {
    if let Ok(metadata) = metadata(&file_path) {
        if let Ok(systime) = metadata.created() {
            let datetime = systime.duration_since(UNIX_EPOCH).expect("Time went backwards");
            let local_datetime = chrono::Local.timestamp(datetime.as_secs() as i64, datetime.subsec_nanos());
            // 使用新的时间格式
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
                    fs::rename(file_path, new_path.clone()).expect("Could not rename file");
                    println!("File renamed to {:?}", new_path);
                }
            }
        }
    }
}





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

    let mut watcher = recommended_watcher(move |res: NotifyResult<Event>| {
        match res {
            Ok(event) => {
                println!("{:?}", event);
                if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                    if let Some(path) = event.paths.get(0) {
                        change_filename_based_on_creation_time(path);
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    for event in rx {
        println!("{:?}", event);
    }

    Ok(())
}


fn main() -> NotifyResult<()> {
    let path = "../../../downloads/抖音直播/GAGAZHANG女装";
    watch_directory(path)
}
