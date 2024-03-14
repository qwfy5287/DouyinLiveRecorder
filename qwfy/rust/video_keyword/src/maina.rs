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

// fn split_video(input_video: &str, output_dir: &str, items: &[Item]) {
//     for (index, item) in items.iter().enumerate() {
//         let output_file = format!("{}/clip_{}.mp4", output_dir, index + 1);
//         println!("Processing clip {}: {}", index + 1, item.keyword);

//         let start_time = item.start_time.replace(",", ".");
//         let end_time = format!(
//             "{:.3}",
//             item.end_time.replace(",", ".").parse::<f32>().unwrap() + 0.5
//         );

//         let output = Command::new("ffmpeg")
//             .args(&[
//                 "-i",
//                 input_video,
//                 "-ss",
//                 &start_time,
//                 "-to",
//                 &end_time,
//                 "-c:v",
//                 "libx264",
//                 "-c:a",
//                 "aac",
//                 "-copyts",
//                 &output_file,
//             ])
//             .output()
//             .expect("Failed to execute ffmpeg command");

//         if output.status.success() {
//             println!("Clip {} generated successfully", index + 1);
//         } else {
//             let stderr = String::from_utf8_lossy(&output.stderr);
//             println!("Failed to generate clip {}. Error: {}", index + 1, stderr);
//         }
//     }
// }

fn split_video(input_video: &str, output_dir: &str, items: &[Item]) {
    for (index, item) in items.iter().enumerate() {
        let output_file = format!("{}/clip_{}.mp4", output_dir, index + 1);
        println!("Processing clip {}: {}", index + 1, item.keyword);

        let start_time = parse_time(&item.start_time);
        let end_time = parse_time(&item.end_time) + 0.5;

        let output = Command::new("ffmpeg")
            .args(&[
                "-i",
                input_video,
                "-ss",
                &start_time.to_string(),
                "-to",
                &end_time.to_string(),
                "-c:v",
                "libx264",
                "-c:a",
                "aac",
                "-copyts",
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

fn parse_time(time: &str) -> f32 {
    let parts: Vec<&str> = time.split(':').collect();
    let hours = parts[0].parse::<f32>().unwrap();
    let minutes = parts[1].parse::<f32>().unwrap();
    let seconds = parts[2].replace(",", ".").parse::<f32>().unwrap();
    hours * 3600.0 + minutes * 60.0 + seconds
}

fn main() {
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
