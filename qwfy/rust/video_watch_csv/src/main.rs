use notify::{Config, PollWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::process::Command;
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

struct LoggingObserver;

impl Observer for LoggingObserver {
    fn update(&self, event: &Result<notify::Event>) {
        println!("{:?}", event);
    }
}



fn main() {
    let downloads_path = "/Users/qwfy/Downloads";

    let mut file_watcher = FileWatcher::new();
    let logging_observer = LoggingObserver; // 现有的观察者

    file_watcher.register(Box::new(logging_observer));
    file_watcher.start(&downloads_path); // 传递路径变量
}
