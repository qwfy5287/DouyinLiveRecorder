use notify::{Config, PollWatcher, RecursiveMode, Result, Watcher};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::PathBuf;

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
            if let Some(path) = self.get_csv_file_path(event) {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                
                if self.should_skip_processing(file_name) {
                    return;
                }
                
                if let Some(name) = self.get_monitored_name(file_name) {
                    let latest_version = self.find_latest_source_version(name);
                    let latest_source_path = self.get_source_file_path(name, latest_version);
                    
                    if latest_source_path.exists() {
                        self.process_existing_source_file(name, latest_version, &latest_source_path, path);
                    } else {
                        self.process_new_source_file(name, path);
                    }
                }
            }
        }
    }
    
    fn get_csv_file_path<'a>(&'a self, event: &'a notify::Event) -> Option<&'a Path> {
        if event.kind.is_create() {
            event.paths.get(0).filter(|path| path.extension().unwrap_or_default() == "csv").map(|path| path.as_path())
        } else {
            None
        }
    }
    
    fn should_skip_processing(&self, file_name: &str) -> bool {
        file_name.contains("_source_")
    }
    
    fn get_monitored_name(&self, file_name: &str) -> Option<&str> {
        self.monitored_names.iter().find(|name| file_name.contains(&**name)).map(|s| s.as_str())
    }
    
    fn get_source_file_path(&self, name: &str, version: u32) -> PathBuf {
        Path::new(&self.path).join(format!("{}_source_v{}.csv", name, version))
    }
    
    fn process_existing_source_file(&self, name: &str, latest_version: u32, latest_source_path: &Path, new_csv_path: &Path) {
        if self.compare_csv_files(latest_source_path, new_csv_path) {
            if let Err(err) = fs::remove_file(new_csv_path) {
                eprintln!("Failed to remove file: {:?}", err);
            }
        } else {
            let new_version = latest_version + 1;
            let new_source_path = self.get_source_file_path(name, new_version);
            if let Err(err) = fs::rename(new_csv_path, &new_source_path) {
                eprintln!("Failed to rename file: {:?}", err);
            } else {
                self.notify_new_source_file(&new_source_path);
            }
        }
    }
    
    fn process_new_source_file(&self, name: &str, new_csv_path: &Path) {
        let source_v1_path = self.get_source_file_path(name, 1);
        if let Err(err) = fs::rename(new_csv_path, &source_v1_path) {
            eprintln!("Failed to rename file: {:?}", err);
        } else {
            self.notify_new_source_file(&source_v1_path);
        }
    }

    fn find_latest_source_version(&self, name: &str) -> u32 {
        let mut latest_version = 1;
        loop {
            let source_path = Path::new(&self.path).join(format!("{}_source_v{}.csv", name, latest_version));
            if !source_path.exists() {
                break;
            }
            latest_version += 1;
        }
        latest_version - 1
    }

    fn notify_new_source_file(&self, path: &Path) {
        println!("New source file generated: {}", path.display());
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