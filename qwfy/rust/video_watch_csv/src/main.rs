
// 

use notify::{Config, PollWatcher, RecursiveMode, Result, Watcher};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

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

struct CsvObserver {
    path: String,
    monitored_names: Vec<String>,
}

impl CsvObserver {
    fn new(path: &str, monitored_names: Vec<String>) -> Self {
        Self {
            path: path.to_string(),
            monitored_names,
        }
    }

    fn process_csv(&self, event: &Result<notify::Event>) {
        if let Ok(event) = event {
            if event.kind.is_create() {
                if let Some(path) = event.paths.get(0) {
                    if path.extension().unwrap_or_default() == "csv" {
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        for name in &self.monitored_names {
                            if file_name.contains(name) {
                                let source_v1_path = Path::new(&self.path).join(format!("{}_source_v1.csv", name));
                                let new_csv_path = path;

                                if source_v1_path.exists() {
                                    if self.compare_csv_files(&source_v1_path, new_csv_path) {
                                        // 文件内容相同,删除新文件,但不删除包含 "_source_" 的文件
                                        if !file_name.contains("_source_") {
                                            if let Err(err) = fs::remove_file(new_csv_path) {
                                                eprintln!("Failed to remove file: {:?}", err);
                                            }
                                        }
                                    } else {
                                        // 文件内容不同,将新文件重命名为 source_v2.csv
                                        let source_v2_path = Path::new(&self.path).join(format!("{}_source_v2.csv", name));
                                        if let Err(err) = fs::rename(new_csv_path, &source_v2_path) {
                                            eprintln!("Failed to rename file: {:?}", err);
                                        }
                                    }
                                } else {
                                    // source_v1.csv 不存在,将新文件重命名为 source_v1.csv
                                    if let Err(err) = fs::rename(new_csv_path, &source_v1_path) {
                                        eprintln!("Failed to rename file: {:?}", err);
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn compare_csv_files(&self, file1: &Path, file2: &Path) -> bool {
        let file1_reader = match File::open(file1) {
            Ok(file) => BufReader::new(file),
            Err(_) => return false,
        };
    
        let file2_reader = match File::open(file2) {
            Ok(file) => BufReader::new(file),
            Err(_) => return false,
        };
    
        file1_reader
            .lines()
            .zip(file2_reader.lines())
            .all(|(line1, line2)| match (line1, line2) {
                (Ok(line1), Ok(line2)) => line1 == line2,
                _ => false,
            })
    }
}

impl Observer for CsvObserver {
    fn update(&self, event: &Result<notify::Event>) {
        self.process_csv(event);
    }
}

fn main() {
    let downloads_path = "/Users/qwfy/Downloads";
    let mut file_watcher = FileWatcher::new();
    let monitored_names = vec!["刘一一".to_string(), "魏老板".to_string()];
    let csv_observer = CsvObserver::new(downloads_path, monitored_names);

    file_watcher.register(Box::new(csv_observer));
    file_watcher.start(downloads_path);
}