// qwfy/rust/video_splitter/src/main.rs

// use std::process::Command;
// use std::sync::Arc;
// use std::thread;

// fn split_video(file_name: &str, segment_duration: u32, output_format: &str) {
//     // 计算视频总长度并确定需要分割成多少段
//     // 注意: 这里需要一个方法来获取视频的总长度，可能需要调用 ffmpeg 的 ffprobe
//     let video_length = 600; // 假设视频长度是 600 秒
//     let segments = video_length / segment_duration;

//     // 创建 Arc 包装的字符串，这样它就可以安全地在多个线程之间共享
//     let file_name = Arc::new(file_name.to_string());
//     let output_format = Arc::new(output_format.to_string());

//     // 并发地进行分割操作
//     let mut handles = vec![];
//     for i in 0..segments {
//         let file_name = Arc::clone(&file_name);
//         let output_format = Arc::clone(&output_format);

//         let handle = thread::spawn(move || {
//             // 计算每段视频的开始时间
//             let start_time = i * segment_duration;
//             // 构建输出文件名
//             let output = format!("{}_segment_{:02}.{}", file_name, i, output_format);

//             // 调用 FFmpeg 进行视频分割
//             let status = Command::new("ffmpeg")
//                 .arg("-ss")
//                 .arg(format!("{}", start_time)) // 设置开始时间
//                 .arg("-i")
//                 .arg(&*file_name) // 输入文件
//                 .arg("-t")
//                 .arg(format!("{}", segment_duration)) // 设置持续时间
//                 .arg("-c")
//                 .arg("copy") // 使用相同的编码进行复制
//                 .arg(output) // 输出文件名
//                 .status()
//                 .expect("failed to execute ffmpeg");

//             assert!(status.success());
//         });

//         handles.push(handle);
//     }

//     // 等待所有线程完成
//     for handle in handles {
//         handle.join().unwrap();
//     }
// }

// fn main() {
//     split_video(
//         "./data/朴夫人_2024-02-24_19-40-06/朴夫人_2024-02-24_19-40-06_007.mp4",
//         10,
//         "mp4",
//     );
// }

use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::thread;

fn split_video(file_path: &str, segment_duration: u32, output_format: &str) {
    // Assume video length is 600 seconds, which should be obtained using ffprobe
    let video_length = 600;
    let segments = video_length / segment_duration;

    // Parse the file name and directory
    let path = Path::new(file_path);
    let file_stem = path
        .file_stem()
        .expect("Failed to read file stem")
        .to_str()
        .expect("Failed to convert OsStr to String")
        .to_string();
    let parent_dir = path.parent().expect("Failed to get parent directory");

    // Create a sub-directory based on the file stem
    let sub_dir_path = parent_dir.join(&file_stem);
    create_dir_all(&sub_dir_path).expect("Failed to create subdirectory");

    // Shared among threads
    let file_path = Arc::new(file_path.to_string());
    let output_format = Arc::new(output_format.to_string());

    // Perform the splitting operation concurrently
    let mut handles = vec![];
    for i in 0..segments {
        let file_path_clone = Arc::clone(&file_path);
        let output_format_clone = Arc::clone(&output_format);
        let sub_dir_path_clone = sub_dir_path.clone();
        let file_stem_clone = file_stem.clone();

        let handle = thread::spawn(move || {
            // Calculate the start time for each video segment
            let start_time = i * segment_duration;

            // Build the output file name, outputting to the subdirectory
            let output_path = PathBuf::from(sub_dir_path_clone).join(format!(
                "{}_segment_{:02}.{}",
                &file_stem_clone, i, *output_format_clone
            ));

            // Invoke FFmpeg to split the video
            let status = Command::new("ffmpeg")
                .arg("-y") // 自动确认覆盖
                .args(&["-ss", &start_time.to_string()])
                .args(&["-i", &*file_path_clone])
                .args(&["-t", &segment_duration.to_string()])
                .args(&["-c", "copy"])
                .arg(output_path) // Output file path
                .status()
                .expect("Failed to execute ffmpeg");

            assert!(status.success(), "FFmpeg command failed");
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

// fn main() {
//     let input_video_path = "./data/朴夫人_2024-02-24_19-40-06/朴夫人_2024-02-24_19-40-06_007.mp4";
//     let segment_duration = 10; // seconds
//     let output_format = "mp4"; // Output format

//     split_video(input_video_path, segment_duration, output_format);
// }
//

fn main() {
    let directory_path = "./data/朴夫人_2024-02-24_19-40-06"; // 设置目录路径
    let segment_duration = 10; // 段落持续时间，以秒为单位
    let output_format = "mp4"; // 输出格式

    // 读取目录中的文件列表
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // 确保是文件且扩展名符合要求
                if path.is_file()
                    && path.extension().and_then(|s| s.to_str()) == Some(output_format)
                {
                    // 你可能需要更改条件来过滤合适的视频文件

                    if let Some(video_path) = path.to_str() {
                        split_video(video_path, segment_duration, output_format);
                    }
                }
            }
        }
    }
}

// split_video 函数定义保持不变，确保它在本文件中或者是导入的模块中。
