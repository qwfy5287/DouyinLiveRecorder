use std::process::Command;
use std::sync::Arc;
use std::thread;

fn split_video(file_name: &str, segment_duration: u32, output_format: &str) {
    // 计算视频总长度并确定需要分割成多少段
    // 注意: 这里需要一个方法来获取视频的总长度，可能需要调用 ffmpeg 的 ffprobe
    let video_length = 600; // 假设视频长度是 600 秒
    let segments = video_length / segment_duration;

    // 创建 Arc 包装的字符串，这样它就可以安全地在多个线程之间共享
    let file_name = Arc::new(file_name.to_string());
    let output_format = Arc::new(output_format.to_string());

    // 并发地进行分割操作
    let mut handles = vec![];
    for i in 0..segments {
        let file_name = Arc::clone(&file_name);
        let output_format = Arc::clone(&output_format);

        let handle = thread::spawn(move || {
            // 计算每段视频的开始时间
            let start_time = i * segment_duration;
            // 构建输出文件名
            let output = format!("{}_segment_{}.{}", file_name, i, output_format);

            // 调用 FFmpeg 进行视频分割
            let status = Command::new("ffmpeg")
                .arg("-ss").arg(format!("{}", start_time)) // 设置开始时间
                .arg("-i").arg(&*file_name) // 输入文件
                .arg("-t").arg(format!("{}", segment_duration)) // 设置持续时间
                .arg("-c").arg("copy") // 使用相同的编码进行复制
                .arg(output) // 输出文件名
                .status()
                .expect("failed to execute ffmpeg");

            assert!(status.success());
        });

        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    split_video("./data/朴夫人_2024-02-24_19-40-06_006.mp4_chunk_50.mp4", 30, "mp4");
}
