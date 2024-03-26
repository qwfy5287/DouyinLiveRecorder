use std::fs;
use std::path::Path;
use serde::Serialize;
// use std::process;

use video_login::common::login_common::{login, handle_login_result};

#[derive(Serialize)]
struct FileInfo {
    id: String,
    file_name: String,
    first_folder: String,
    second_folder: String, 
    third_folder: String,
    fourth_folder: String,
}

struct FileInfoBuilder {
    id: String,
    file_name: String,
    first_folder: String,
    second_folder: String,
    third_folder: String,
    fourth_folder: String,
}

impl FileInfoBuilder {
    fn new(id: String, file_name: String) -> FileInfoBuilder {
        FileInfoBuilder {
            id,
            file_name,
            first_folder: String::new(),
            second_folder: String::new(),
            third_folder: String::new(),
            fourth_folder: String::new(),
        }
    }
    
    fn first_folder(mut self, folder: String) -> FileInfoBuilder {
        self.first_folder = folder;
        self
    }
    
    fn second_folder(mut self, folder: String) -> FileInfoBuilder {
        self.second_folder = folder;
        self
    }
    
    fn third_folder(mut self, folder: String) -> FileInfoBuilder {
        self.third_folder = folder;
        self
    }
    
    fn fourth_folder(mut self, folder: String) -> FileInfoBuilder {
        self.fourth_folder = folder;
        self
    }
    
    fn build(self) -> FileInfo {
        FileInfo { 
            id: self.id,
            file_name: self.file_name,
            first_folder: self.first_folder,
            second_folder: self.second_folder,
            third_folder: self.third_folder,
            fourth_folder: self.fourth_folder,
        }
    }
}

trait FileFilter {
    fn filter(&self, file_name: &str) -> bool;
}

struct JpgFilter;

impl FileFilter for JpgFilter {
    fn filter(&self, file_name: &str) -> bool {
        file_name.ends_with(".jpg")
    }
}

trait FileSorter {
    fn sort(&self, file_infos: &mut [FileInfo]);
}

struct FileNameAscSorter;

impl FileSorter for FileNameAscSorter {
    fn sort(&self, file_infos: &mut [FileInfo]) {
        file_infos.sort_by(|a, b| a.file_name.cmp(&b.file_name));
    }
}

fn traverse_directory(dir: &Path, file_filter: &dyn FileFilter, file_infos: &mut Vec<FileInfo>, 
                      id_counter: &mut i32, current_folders: &[String; 4]) {
    for entry in fs::read_dir(dir).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = entry.file_name().into_string().unwrap_or_default();
                if !folder_name.starts_with('.') {
                    let mut new_folders = current_folders.clone();
                    new_folders[current_folders.iter().position(|f| f.is_empty()).unwrap()] = folder_name;
                    traverse_directory(&path, file_filter, file_infos, id_counter, &new_folders);
                }
            } else if path.is_file() {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                if file_filter.filter(&file_name) {
                    let file_info = FileInfoBuilder::new(format!("{:05}", *id_counter), file_name)
                        .first_folder(current_folders[0].clone())  
                        .second_folder(current_folders[1].clone())
                        .third_folder(current_folders[2].clone())
                        .fourth_folder(current_folders[3].clone())
                        .build();
                    file_infos.push(file_info);
                    *id_counter += 1;
                }
            }
        }
    }
}


#[tokio::main]
async fn main() {
    let login_result = login("18250833087".to_string(), "qwfy@123!456".to_string()).await;
    handle_login_result(login_result);

    let args: Vec<String> = std::env::args().collect();
    let root_dir = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new("/Users/qwfy/douyin-thumb")
    };

    let mut file_infos: Vec<FileInfo> = Vec::new();
    let mut id_counter = 1;
    let file_filter = JpgFilter;
    let file_sorter = FileNameAscSorter;
    traverse_directory(root_dir, &file_filter, &mut file_infos, &mut id_counter, &[String::new(), String::new(), String::new(), String::new()]);
    file_sorter.sort(&mut file_infos);
    let json = serde_json::to_string_pretty(&file_infos).unwrap();

    // 根据不同的运行环境决定输出文件路径
    let output_path = if cfg!(debug_assertions) {
        // cargo run 时
        let output_file = "output.json";
        std::path::Path::new(output_file).to_path_buf()
    } else {
        // 运行打包的可执行文件时
        let current_exe = std::env::current_exe().expect("无法获取当前可执行文件路径");
        let current_dir = current_exe.parent().expect("无法获取当前目录");
        current_dir.join("output.json")
    };

    // 将JSON写入文件
    fs::write(&output_path, json).expect("无法写入文件");
    println!("JSON已保存到 {:?}", output_path);
}
