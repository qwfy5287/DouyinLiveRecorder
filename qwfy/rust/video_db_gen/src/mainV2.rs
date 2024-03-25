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

struct FileInfoBuilder {
    id: String,
    file_name: String,
    first_folder: String,
    second_folder: String,
    third_folder: String,
    fourth_folder: String,
}

impl FileInfoBuilder {
    fn new(id: String, file_name: String) -> Self {
        FileInfoBuilder {
            id,
            file_name,
            first_folder: String::new(),
            second_folder: String::new(),
            third_folder: String::new(),
            fourth_folder: String::new(),
        }
    }

    fn first_folder(mut self, folder: String) -> Self {
        self.first_folder = folder;
        self
    }

    fn second_folder(mut self, folder: String) -> Self {
        self.second_folder = folder;
        self
    }

    fn third_folder(mut self, folder: String) -> Self {
        self.third_folder = folder;
        self
    }

    fn fourth_folder(mut self, folder: String) -> Self {
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

trait FileInfoVisitor {
    fn visit_file(&mut self, file_info: FileInfo);
}

struct FileInfoCollector {
    file_infos: Vec<FileInfo>,
}

impl FileInfoCollector {
    fn new() -> Self {
        FileInfoCollector {
            file_infos: Vec::new(),
        }
    }
}

impl FileInfoVisitor for FileInfoCollector {
    fn visit_file(&mut self, file_info: FileInfo) {
        self.file_infos.push(file_info);
    }
}

fn visit_directory<V: FileInfoVisitor>(
    dir: &Path,
    visitor: &mut V,
    id_counter: &mut i32,
    first_folder: String,
    second_folder: String,
    third_folder: String,
    fourth_folder: String,
) {
    for entry in fs::read_dir(dir).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let folder = entry.file_name().into_string().unwrap_or_default();
                if folder.starts_with('.') {
                    continue;
                }
                println!("Processing directory: {:?}", path);
                visit_directory(
                    &path,
                    visitor,
                    id_counter,
                    first_folder.clone(),
                    second_folder.clone(),
                    third_folder.clone(),
                    folder,
                );
            } else if path.is_file() {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
                if extension.eq_ignore_ascii_case("jpg") {
                    let file_info = FileInfoBuilder::new(format!("{:05}", *id_counter), file_name)
                        .first_folder(first_folder.clone())
                        .second_folder(second_folder.clone())
                        .third_folder(third_folder.clone())
                        .fourth_folder(fourth_folder.clone())
                        .build();
                    visitor.visit_file(file_info);
                    *id_counter += 1;
                }
            }
        }
    }
}

fn main() {
    let root_dir = Path::new("../../../downloads");
    let mut id_counter = 1;
    let mut collector = FileInfoCollector::new();

    visit_directory(
        root_dir,
        &mut collector,
        &mut id_counter,
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    );

    collector.file_infos.sort_by(|a, b| a.file_name.cmp(&b.file_name));

    let json = serde_json::to_string_pretty(&collector.file_infos).unwrap();
    println!("{}", json);
}