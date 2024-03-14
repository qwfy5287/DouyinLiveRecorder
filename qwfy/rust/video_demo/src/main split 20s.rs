use ffmpeg_next as ffmpeg;
use std::env;
use std::path::Path;

fn main() {
    ffmpeg::init().unwrap();

    let input = env::args().nth(1).expect("请提供输入文件路径");
    let input = Path::new(&input);

    let mut ictx = ffmpeg::format::input(&input).unwrap();
    let input_stream_index = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .unwrap()
        .index();

    let input_stream = ictx.stream(input_stream_index).unwrap();
    let input_stream_params = input_stream.parameters();
    let metadata = ictx.metadata().to_owned();

    let segment_time = 20 * 1_000_000; // 20 seconds
    let mut current_segment_ts = 0;
    let mut segment_index = 1;

    let mut octx = ffmpeg::format::output(&format!(
        "{}_part_{:03}.mp4",
        input.file_stem().unwrap().to_str().unwrap(),
        segment_index
    ))
    .unwrap();
    let encoder = ffmpeg::encoder::find_by_name("libx264").unwrap();
    let mut output_stream = octx.add_stream(encoder).unwrap();
    output_stream.set_parameters(input_stream_params.clone());

    let mut ost_time_base = output_stream.time_base();

    octx.set_metadata(metadata.clone());
    octx.write_header().unwrap();

    for (stream, mut packet) in ictx.packets() {
        if stream.index() == input_stream_index {
            let pts = packet.pts();
            let dts = packet.dts();
            let duration = packet.duration();

            if pts.is_some() && dts.is_some() && duration > 0 {
                if pts.unwrap() >= current_segment_ts + segment_time {
                    octx.write_trailer().unwrap();

                    current_segment_ts += segment_time;
                    segment_index += 1;

                    octx = ffmpeg::format::output(&format!(
                        "{}_part_{:03}.mp4",
                        input.file_stem().unwrap().to_str().unwrap(),
                        segment_index
                    ))
                    .unwrap();
                    output_stream = octx.add_stream(encoder).unwrap();
                    output_stream.set_parameters(input_stream_params.clone());

                    ost_time_base = output_stream.time_base();

                    octx.set_metadata(metadata.clone());
                    octx.write_header().unwrap();
                }

                packet.rescale_ts(stream.time_base(), ost_time_base);
                packet.set_position(-1);
                packet.set_stream(0);

                if let Err(error) = packet.write_interleaved(&mut octx) {
                    eprintln!("Error writing packet: {}", error);
                }
            } else {
                eprintln!("Skipping invalid packet");
            }
        }
    }

    octx.write_trailer().unwrap();
}
