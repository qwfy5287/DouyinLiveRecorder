use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[derive(Deserialize)]
struct Item {
    keyword: String,
    text: String,
    start_time: String,
    end_time: String,
}

fn split_video(input_video: &str, output_dir: &str, items: &[Item]) {
    for (index, item) in items.iter().enumerate() {
        let output_file = format!("{}/clip_{}.mp4", output_dir, index + 1);
        println!("Processing clip {}: {}", index + 1, item.keyword);

        // 不需要将时间格式转换，因为新的格式已经是适合的格式
        let start_time = item.start_time.replace(",", ".");
        let end_time = item.end_time.replace(",", ".");

        // let output = Command::new("ffmpeg")
        //     .args(&[
        //         "-i",
        //         input_video,
        //         "-ss",
        //         &start_time,
        //         "-to",
        //         &end_time,
        //         "-c",
        //         "copy",
        //         &output_file,
        //     ])
        //     .output()
        //     .expect("Failed to execute ffmpeg command");

        let output = Command::new("ffmpeg")
            .args(&[
                "-i",
                input_video,
                "-ss",
                &start_time,
                "-to",
                &end_time,
                "-c:v",
                "libx264", // 指定视频编解码器为 H.264
                "-c:a",
                "aac",     // 指定音频编解码器为 AAC
                "-strict", // 有时需要对某些编解码器使用 strict 选项
                "experimental",
                &output_file,
            ])
            .output()
            .expect("Failed to execute ffmpeg command");

        if output.status.success() {
            println!("Clip {} generated successfully", index + 1);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Failed to generate clip {}. Error: {}", index + 1, stderr);
        }
    }
}

fn main() {
    // 新的 JSON 文件路径
    let json_file_path = "./data/奇缘.json";

    let mut file = File::open(json_file_path).expect("Failed to open JSON file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read JSON file");

    let items: Vec<Item> = serde_json::from_str(&json_data).unwrap();

    println!("Splitting video...");
    split_video("./data/奇缘.mp4", "./output", &items);
    println!("Video splitting completed.");
}
