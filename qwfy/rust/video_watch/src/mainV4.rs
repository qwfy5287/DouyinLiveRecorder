// qwfy/rust/video_watch/src/main.rs

use notify::{Config, PollWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

mod common {
    pub mod file_common;
    pub mod thumb;
    pub mod thumb_next;
}

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

impl Observer for Mp4Observer {
    fn update(&self, event: &Result<notify::Event>) {
        if let Ok(event) = event {
            for path in &event.paths {
                if let Some(extension) = path.extension() {
                    if extension == "mp4" {
                        if event.kind.is_create() {
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
                        } else if event.kind.is_modify() {
                            process_video(&path);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let downloads_path = "../../../downloads/抖音直播";

    // 在开始监控之前，重命名目录下现有的所有.mp4文件
    rename_existing_files(Path::new(&downloads_path)).expect("Failed to rename existing files");

    let mut file_watcher = FileWatcher::new();
    let logging_observer = LoggingObserver; // 现有的观察者
    let mp4_observer = Mp4Observer; // 新的观察者

    file_watcher.register(Box::new(logging_observer));
    file_watcher.register(Box::new(mp4_observer)); // 注册新观察者
    file_watcher.start(&downloads_path); // 传递路径变量
}
