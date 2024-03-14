use std::process::Command;

fn split_mp4(input_file: &str, output_prefix: &str) {
    let mut segment_index = 0;

    loop {
        let start_time = segment_index * 20;
        let output_file = format!("{}_{}.mp4", output_prefix, segment_index);

        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_file)
            .arg("-ss")
            .arg(start_time.to_string()) // Start at 20s intervals
            .arg("-t")
            .arg("20") // 20-second duration
            .arg("-c:v")
            .arg("copy") // Copy video stream without re-encoding
            .arg("-c:a")
            .arg("copy") // Copy audio stream without re-encoding
            .arg(output_file)
            .status()
            .expect("Failed to execute FFmpeg");

        if !status.success() {
            break; // Stop if a segment could not be created
        }

        segment_index += 1;
    }
}

fn main() {
    let input_video = "./data/example.mp4";
    let output_prefix = "example_segment";
    split_mp4(input_video, output_prefix);
}
