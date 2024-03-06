use ffmpeg_next as ffmpeg;
use std::path::Path;

pub fn generate_thumbnails(video_path: &Path, output_pattern: &str) -> Result<(), String> {
    ffmpeg::init().map_err(|e| format!("Failed to initialize FFmpeg: {}", e))?;

    let mut input_context = ffmpeg::format::input(&video_path)
        .map_err(|e| format!("Failed to open input file: {}", e))?;
    let input = input_context
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or_else(|| "No video stream found".to_string())?;

    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())
        .map_err(|e| format!("Failed to create decoder context: {}", e))?;
    let mut decoder = context_decoder
        .decoder()
        .video()
        .map_err(|e| format!("Failed to create video decoder: {}", e))?;

    let mut thumbnail_index = 0;
    let thumbnail_interval = 15.0; // 每15秒一张缩略图
    let mut next_thumbnail_timestamp = thumbnail_interval;

    for (stream, packet) in input_context.packets() {
        if stream.index() == video_stream_index {
            decoder
                .send_packet(&packet)
                .map_err(|e| format!("Failed to send packet to decoder: {}", e))?;

            let mut decoded = ffmpeg::frame::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let timestamp =
                    decoded.timestamp().unwrap_or(0) as f64 * f64::from(stream.time_base());

                if timestamp >= next_thumbnail_timestamp {
                    let output_filename =
                        output_pattern.replace("%03d", &format!("{:03}", thumbnail_index));
                    let output_path = Path::new(&output_filename);

                    let mut output = ffmpeg::format::output(&output_path)
                        .map_err(|e| format!("Failed to create output context: {}", e))?;

                    let mut output_stream = output
                        .add_stream(ffmpeg::encoder::find(ffmpeg::codec::Id::MJPEG))
                        .map_err(|e| format!("Failed to create output stream: {}", e))?;

                    let mut output_encoder_context =
                        ffmpeg::codec::context::Context::from_parameters(
                            output_stream.parameters(),
                        )
                        .map_err(|e| format!("Failed to create encoder context: {}", e))?;
                    let mut output_encoder = output_encoder_context
                        .encoder()
                        .video()
                        .map_err(|e| format!("Failed to create video encoder: {}", e))?;

                    output_encoder.set_height(decoded.height());
                    output_encoder.set_width(decoded.width());
                    output_encoder.set_format(ffmpeg::format::Pixel::YUVJ420P);
                    output_encoder.set_time_base((1, 1));

                    let mut output_encoder = output_encoder
                        .open_as(ffmpeg::codec::Id::MJPEG)
                        .map_err(|e| format!("Failed to open video encoder: {}", e))?;

                    let mut encoder_frame = ffmpeg::frame::Video::empty();
                    encoder_frame.set_format(ffmpeg::format::Pixel::YUVJ420P);
                    encoder_frame.set_width(decoded.width());
                    encoder_frame.set_height(decoded.height());

                    ffmpeg::software::scaling::context::Context::get(
                        decoded.format(),
                        decoded.width(),
                        decoded.height(),
                        encoder_frame.format(),
                        encoder_frame.width(),
                        encoder_frame.height(),
                        ffmpeg::software::scaling::flag::Flags::BILINEAR,
                    )
                    .map_err(|e| format!("Failed to get scaling context: {}", e))?
                    .run(&decoded, &mut encoder_frame)
                    .map_err(|e| format!("Failed to convert frame: {}", e))?;

                    output_encoder
                        .send_frame(&encoder_frame)
                        .map_err(|e| format!("Failed to send frame to encoder: {}", e))?;

                    output
                        .write_header()
                        .map_err(|e| format!("Failed to write output header: {}", e))?;

                    let mut output_packet = ffmpeg::codec::packet::Packet::empty();
                    output_encoder
                        .receive_packet(&mut output_packet)
                        .map_err(|e| format!("Failed to receive packet from encoder: {}", e))?;

                    output_packet
                        .write_interleaved(&mut output)
                        .map_err(|e| format!("Failed to write packet: {}", e))?;

                    output
                        .write_trailer()
                        .map_err(|e| format!("Failed to write output trailer: {}", e))?;

                    thumbnail_index += 1;
                    next_thumbnail_timestamp += thumbnail_interval;
                }
            }
        }
    }

    decoder
        .send_eof()
        .map_err(|e| format!("Failed to send EOF to decoder: {}", e))?;

    decoder.flush();

    Ok(())
}
