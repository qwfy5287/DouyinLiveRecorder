// /Users/qwfy/GitWork/DouyinLiveRecorder/downloads

// [
//     {
//       id: '00001',
//       fileName: '@魏老板私服_2024-03-14_07-03-35_000_00_00_15.jpg',
//       firstFolder: '抖音直播',
//       secondFolder: '@魏老板私服',
//       thirdFolder: '@魏老板私服_2024-03-14',
//       fourthFolder: '@魏老板私服_2024-03-14_07-03-35_000_thumb'
//     },
//     {
//       id: '00002',
//       fileName: '@魏老板私服_2024-03-14_07-03-35_000_00_00_30.jpg',
//       firstFolder: '抖音直播',
//       secondFolder: '@魏老板私服',
//       thirdFolder: '@魏老板私服_2024-03-14',
//       fourthFolder: '@魏老板私服_2024-03-14_07-03-35_000_thumb'
//     },
// ]


use std::fs;
use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
struct FileInfo {
    id: String,
    file_name: String,
    first_folder: String,
    second_folder: String,
    third_folder: String,
    fourth_folder: String,
}

fn main() {
    let root_dir = Path::new("../../../downloads");
    let mut file_infos = Vec::new();
    let mut id_counter = 1;

    for entry in fs::read_dir(root_dir).unwrap() {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                let first_folder = entry.file_name().into_string().unwrap_or_default();
                if first_folder.starts_with('.') {
                    continue;
                }
                println!("Processing directory: {:?}", entry.path());

                for sub_entry in fs::read_dir(entry.path()).unwrap() {
                    if let Ok(sub_entry) = sub_entry {
                        if sub_entry.path().is_dir() {
                            let second_folder = sub_entry.file_name().into_string().unwrap_or_default();
                            println!("Processing directory: {:?}", sub_entry.path());

                            for sub_sub_entry in fs::read_dir(sub_entry.path()).unwrap() {
                                if let Ok(sub_sub_entry) = sub_sub_entry {
                                    if sub_sub_entry.path().is_dir() {
                                        let third_folder = sub_sub_entry.file_name().into_string().unwrap_or_default();
                                        println!("Processing directory: {:?}", sub_sub_entry.path());

                                        for sub_sub_sub_entry in fs::read_dir(sub_sub_entry.path()).unwrap() {
                                            if let Ok(sub_sub_sub_entry) = sub_sub_sub_entry {
                                                if sub_sub_sub_entry.path().is_dir() {
                                                    let fourth_folder = sub_sub_sub_entry.file_name().into_string().unwrap_or_default();
                                                    println!("Processing directory: {:?}", sub_sub_sub_entry.path());

                                                    for file in fs::read_dir(sub_sub_sub_entry.path()).unwrap() {
                                                        if let Ok(file) = file {
                                                            if file.path().is_file() {
                                                                let file_name = file.file_name().into_string().unwrap_or_default();
                                                                let file_info = FileInfo {
                                                                    id: format!("{:05}", id_counter),
                                                                    file_name,
                                                                    first_folder: first_folder.clone(),
                                                                    second_folder: second_folder.clone(),
                                                                    third_folder: third_folder.clone(),
                                                                    fourth_folder: fourth_folder.clone(),
                                                                };
                                                                file_infos.push(file_info);
                                                                id_counter += 1;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&file_infos).unwrap();
    println!("{}", json);
}