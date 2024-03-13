// use serde::Deserialize;
// use std::process::Command;

// #[derive(Deserialize)]
// struct Item {
//     keyword: String,
//     snippet: String,
//     start: String,
//     end: String,
// }

// fn split_video(input_video: &str, output_dir: &str, items: &[Item]) {
//     for (index, item) in items.iter().enumerate() {
//         let output_file = format!("{}/clip_{}.mp4", output_dir, index + 1);
//         let _ = Command::new("ffmpeg")
//             .args(&[
//                 "-i",
//                 input_video,
//                 "-ss",
//                 &item.start,
//                 "-to",
//                 &item.end,
//                 "-c",
//                 "copy",
//                 &output_file,
//             ])
//             .output()
//             .expect("Failed to execute ffmpeg command");
//     }
// }

// fn main() {
//     let json_data = r#"
//     [
//         {
//             "keyword": "肚子上肉多",
//             "snippet": "我的肚子上肉肉比较多的",
//             "start": "00:00:01,400",
//             "end": "00:00:03,133"
//         },
//         {
//             "keyword": "苹果型身材",
//             "snippet": "你是苹果型身材的",
//             "start": "00:00:08,800",
//             "end": "00:00:09,600"
//         }
//     ]
//     "#;

//     let items: Vec<Item> = serde_json::from_str(json_data).unwrap();
//     split_video(
//         "./data/GAGAZHANG女装_2024-02-27_17-04-08_002.mp4",
//         "./output",
//         &items,
//     );
// }

use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
struct Item {
    keyword: String,
    snippet: String,
    start: String,
    end: String,
}

// fn split_video(input_video: &str, output_dir: &str, items: &[Item]) {
//     for (index, item) in items.iter().enumerate() {
//         let output_file = format!("{}/clip_{}.mp4", output_dir, index + 1);
//         println!("Processing clip {}", index + 1);
//         let output = Command::new("ffmpeg")
//             .args(&[
//                 "-i",
//                 input_video,
//                 "-ss",
//                 &item.start,
//                 "-to",
//                 &item.end,
//                 "-c",
//                 "copy",
//                 &output_file,
//             ])
//             .output()
//             .expect("Failed to execute ffmpeg command");

//         if output.status.success() {
//             println!("Clip {} generated successfully", index + 1);
//         } else {
//             println!("Failed to generate clip {}", index + 1);
//         }
//     }
// }

// fn split_video(input_video: &str, output_dir: &str, items: &[Item]) {
//     for (index, item) in items.iter().enumerate() {
//         let output_file = format!("{}/clip_{}.mp4", output_dir, index + 1);
//         println!("Processing clip {}", index + 1);
//         let output = Command::new("ffmpeg")
//             .args(&[
//                 "-i",
//                 input_video,
//                 "-ss",
//                 &item.start,
//                 "-to",
//                 &item.end,
//                 "-c",
//                 "copy",
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
        println!("Processing clip {}", index + 1);

        // 将时间格式从 "HH:MM:SS,mmm" 转换为 "HH:MM:SS.mmm"
        let start_time = item.start.replace(",", ".");
        let end_time = item.end.replace(",", ".");

        let output = Command::new("ffmpeg")
            .args(&[
                "-i",
                input_video,
                "-ss",
                &start_time,
                "-to",
                &end_time,
                "-c",
                "copy",
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
    let json_data = r#"
      [
      {
      "keyword": "肚子上肉多",
      "snippet": "我的肚子上肉肉比较多的",
      "start": "00:00:01,400",
      "end": "00:00:03,133"
      },
      {
      "keyword": "苹果型身材",
      "snippet": "你是苹果型身材的",
      "start": "00:00:08,800",
      "end": "00:00:09,600"
      },
      {
      "keyword": "加宽设计",
      "snippet": "因为你看好全部为大家做加宽设计",
      "start": "00:00:22,966",
      "end": "00:00:25,200"
      },
      {
      "keyword": "不会有勒肉感",
      "snippet": "不会有勒肉感",
      "start": "00:00:27,100",
      "end": "00:00:27,733"

      },
      {
      "keyword": "双层压线设计",
      "snippet": "你看双层压线设计的话筋骨感特别强",
      "start": "00:00:30,466",
      "end": "00:00:32,466"
      },
      {
      "keyword": "垫肩",
      "snippet": "有垫肩的宝宝软垫肩",
      "start": "00:00:37,300",
      "end": "00:00:38,500"
      },
      {
      "keyword": "直角肩",
      "snippet": "像是妈生感的直角肩",
      "start": "00:00:40,366",
      "end": "00:00:41,900"
      },
      {
      "keyword": "显头小",
      "snippet": "还有最后10秒钟了显头小的",
      "start": "00:00:43,766",
      "end": "00:00:44,866"
      },
      {
      "keyword": "限时优惠价",
      "snippet": "我这个价格我希望你拍了就忘了",
      "start": "00:00:52,333",
      "end": "00:00:54,133"

      },
      {
      "keyword": "涨价预告",
      "snippet": "之后一定会涨价的",
      "start": "00:00:58,266",
      "end": "00:01:00,000"
      }
      ]
      "#;

    let items: Vec<Item> = serde_json::from_str(json_data).unwrap();

    println!("Splitting video...");
    split_video(
        "./data/GAGAZHANG女装_2024-02-27_17-04-08_002.mp4",
        "./output",
        &items,
    );
    println!("Video splitting completed.");
}
