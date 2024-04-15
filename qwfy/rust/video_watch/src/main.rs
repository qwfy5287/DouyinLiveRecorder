// qwfy/rust/video_watch/src/main.rs

mod common;

use notify::{Config, PollWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::common::file_common::change_filename_based_on_creation_time;
use crate::common::file_common::rename_existing_files;

use crate::common::thumb::process_video;

trait Observer {
    fn update(&self, event: &Result<notify::Event>);
}

struct FileWatcher {
    observers: Vec<Box<dyn Observer>>,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn register(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn notify(&self, event: &Result<notify::Event>) {
        for observer in &self.observers {
            observer.update(event);
        }
    }

    pub fn start(&self, path: &str) {
        let (tx, rx) = channel();

        let config = Config::default().with_poll_interval(Duration::from_secs(2));
        let mut watcher = PollWatcher::new(tx, config).unwrap();

        // 使用传入的路径参数替换原先的硬编码路径
        watcher
            .watch(Path::new(path), RecursiveMode::Recursive)
            .unwrap();

        for event in rx {
            self.notify(&event);
        }
    }
}

struct LoggingObserver;

impl Observer for LoggingObserver {
    fn update(&self, event: &Result<notify::Event>) {
        // println!("{:?}", event);
    }
}

struct Mp4Observer;

impl Mp4Observer {
    fn get_video_duration(path: &Path) -> Result<Duration> {
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
                path.to_str().unwrap(),
            ])
            .output()?;

        let duration_str = String::from_utf8_lossy(&output.stdout);
        let duration_sec = duration_str.trim().parse::<f64>().unwrap_or(0.0);
        Ok(Duration::from_secs_f64(duration_sec))
    }
}

impl Observer for Mp4Observer {
    fn update(&self, event: &Result<notify::Event>) {
        if let Ok(event) = event {
            for path in &event.paths {
                if let Some(extension) = path.extension() {
                    if extension == "mp4" {
                        if event.kind.is_create() {
                            match change_filename_based_on_creation_time(&path) {
                                Ok(new_path) => {
                                    println!("MP4 文件重命名根据创建时间: {:?}", new_path);
                                }
                                Err(e) => {
                                    eprintln!("Error 重命名文件: {:?}", e);
                                }
                            }
                        } else if event.kind.is_modify() {
                            match Self::get_video_duration(path) {
                                Ok(duration) => {
                                    // mp4文件写入完成后，才能获取到视频时长，否则都是 0
                                    if duration.as_secs() > 5 {
                                        // TODO: 先不自动生成缩略图
                                        // process_video(path);
                                    // println!("视频时长: {:?}", duration);


                                    println!("视频路径: {:?}", path);
                                        match copy_file_to_douyin(path.to_str().unwrap()) {
                                            Ok(()) => println!("File copied successfully!"),
                                            Err(e) => println!("Failed to copy file: {}", e),
                                        }
                                    }
                                }
                                Err(e) => eprintln!("Error getting video duration: {:?}", e),
                            }
                        }
                    }
                }
            }
        }
    }
}

// use std::fs;
// // use std::path::Path;

// fn copy_file_to_douyin(file_path: &str) -> Result<()> {
//     let output_root = "/Users/qwfy/douyin-cut";
//     // TODO:  从 file_path 中提取
//     let dest_dir= "/Users/qwfy/douyin-cut"+"/抖音直播/@魏老板私服/@魏老板私服_2024-04-16";
//     let file_name = Path::new(file_path).file_name().unwrap();
//     let dest_path = Path::new(dest_dir).join(file_name);

//     fs::copy(file_path, dest_path)?;
//     Ok(())
// }

// fn main02() {
//     let source_file = "../../../downloads/抖音直播/@魏老板私服/@魏老板私服_2024-04-16_07-28-01_013.mp4";
//     match copy_file_to_douyin(source_file) {
//         Ok(()) => println!("File copied successfully!"),
//         Err(e) => println!("Failed to copy file: {}", e),
//     }
// }

use std::fs;
use std::path::{ PathBuf};

fn copy_file_to_douyin(file_path: &str) -> Result<()> {
    let output_root = "/Users/qwfy/douyin-cut";
    
    // 从 file_path 中提取目录和文件名
    let path = Path::new(file_path);
    let file_name = path.file_name().unwrap();
    
    // 从后面提取目录
    let dest_dir = {
        let components: Vec<_> = path.components().rev().take(2).collect();
        let dest_dir = components.iter().rev().fold(PathBuf::new(), |acc, c| acc.join(c));
        
        // 根据 "_" 分割提取附加的目录层级
        let additional_dir = file_name.to_str().unwrap().split('_').next().unwrap();
        
        Path::new(output_root).join(dest_dir).join(additional_dir)
    };

    println!("目标目录: {:?}", dest_dir);
    
    // // 创建目标目录（如果不存在）
    // fs::create_dir_all(&dest_dir)?;
    
    // let dest_path = dest_dir.join(file_name);
    // fs::copy(file_path, dest_path)?;
    
    Ok(())
}

fn main02() {
    let source_file = "../../../downloads/抖音直播/@魏老板私服/@魏老板私服_2024-04-16_07-28-01_013.mp4";
    
    match copy_file_to_douyin(source_file) {
        Ok(()) => println!("File copied successfully!"),
        Err(e) => println!("Failed to copy file: {}", e),
    }
}

fn main() {
    let downloads_path = "../../../downloads/抖音直播";
    // let downloads_path = "/Volumes/qwfy-wd-2t/抖音直播";

    // 在开始监控之前，重命名目录下现有的所有.mp4文件
    rename_existing_files(Path::new(&downloads_path)).expect("Failed to rename existing files");

    let mut file_watcher = FileWatcher::new();
    let logging_observer = LoggingObserver; // 现有的观察者
    let mp4_observer = Mp4Observer; // 新的观察者

    file_watcher.register(Box::new(logging_observer));
    file_watcher.register(Box::new(mp4_observer)); // 注册新观察者
    file_watcher.start(&downloads_path); // 传递路径变量
}
