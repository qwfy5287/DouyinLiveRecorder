// // // // // // // // // // // // // // // // // // // // use av::codec::context::Context;
// // // // // // // // // // // // // // // // // // // // use av::format::context::Input;
// // // // // // // // // // // // // // // // // // // // use av::frame::Video;
// // // // // // // // // // // // // // // // // // // // use av::media::Type;
// // // // // // // // // // // // // // // // // // // // use av::software::scaling::Context as SwsContext;
// // // // // // // // // // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // // // // // // // // // use std::io::BufWriter;

// // // // // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // // // // // // // // // //     let timestamp = 300; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // // // // // // // // // //     let mut input = Input::new(video_path).unwrap();
// // // // // // // // // // // // // // // // // // // //     let video_stream = input.streams().best(Type::Video).expect("无法找到视频流");

// // // // // // // // // // // // // // // // // // // //     let video_stream_index = video_stream.index();
// // // // // // // // // // // // // // // // // // // //     let context_decoder = Context::from_parameters(video_stream.parameters()).unwrap();

// // // // // // // // // // // // // // // // // // // //     let mut decoder = context_decoder.decoder().video().unwrap();

// // // // // // // // // // // // // // // // // // // //     input.seek(timestamp as i64, ..video_stream_index).unwrap();

// // // // // // // // // // // // // // // // // // // //     for (stream, packet) in input.packets() {
// // // // // // // // // // // // // // // // // // // //         if stream.index() == video_stream_index {
// // // // // // // // // // // // // // // // // // // //             decoder.send_packet(&packet).unwrap();
// // // // // // // // // // // // // // // // // // // //             let mut decoded = Video::empty();
// // // // // // // // // // // // // // // // // // // //             while decoder.receive_frame(&mut decoded).is_ok() {
// // // // // // // // // // // // // // // // // // // //                 let thumbnail = Video::empty();
// // // // // // // // // // // // // // // // // // // //                 let mut sws_context = SwsContext::get(
// // // // // // // // // // // // // // // // // // // //                     decoded.format(),
// // // // // // // // // // // // // // // // // // // //                     decoded.width(),
// // // // // // // // // // // // // // // // // // // //                     decoded.height(),
// // // // // // // // // // // // // // // // // // // //                     decoded.format(),
// // // // // // // // // // // // // // // // // // // //                     decoded.width(),
// // // // // // // // // // // // // // // // // // // //                     decoded.height(),
// // // // // // // // // // // // // // // // // // // //                 )
// // // // // // // // // // // // // // // // // // // //                 .unwrap();

// // // // // // // // // // // // // // // // // // // //                 sws_context.run(&decoded, &thumbnail).unwrap();

// // // // // // // // // // // // // // // // // // // //                 let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // // // // // // // // // //                 thumbnail.write_to(&mut file, Video::jpeg()).unwrap();
// // // // // // // // // // // // // // // // // // // //                 break;
// // // // // // // // // // // // // // // // // // // //             }
// // // // // // // // // // // // // // // // // // // //             break;
// // // // // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // // // // // // // // use std::io::BufWriter;
// // // // // // // // // // // // // // // // // // // use v_frame::frame::Frame;
// // // // // // // // // // // // // // // // // // // use v_frame::pixel::PixelType;

// // // // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // // // // // // // // //     // let thumbnail_path = "path/to/save/thumbnail.jpg";
// // // // // // // // // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // // // // // // // // //     let timestamp = 300; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // // // // // // // // //     let mut frame_reader = v_frame::frame::Reader::from_file(video_path).unwrap();
// // // // // // // // // // // // // // // // // // //     let video_info = frame_reader.video_info().unwrap();

// // // // // // // // // // // // // // // // // // //     let frame_index = (timestamp as f64 * video_info.fps()) as usize;
// // // // // // // // // // // // // // // // // // //     frame_reader.set_frame(frame_index).unwrap();

// // // // // // // // // // // // // // // // // // //     let mut frame = Frame::new(video_info.width(), video_info.height(), PixelType::Rgb24);
// // // // // // // // // // // // // // // // // // //     frame_reader.read_frame(&mut frame).unwrap();

// // // // // // // // // // // // // // // // // // //     let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // // // // // // // // //     frame.write_jpg(&mut file).unwrap();
// // // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // // // // // // // use std::io::BufWriter;
// // // // // // // // // // // // // // // // // // use v_frame::frame::Frame;
// // // // // // // // // // // // // // // // // // use v_frame::pixel::PixelType;
// // // // // // // // // // // // // // // // // // use v_frame::prelude::*;

// // // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // // // // // // // //     // let thumbnail_path = "path/to/save/thumbnail.jpg";
// // // // // // // // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // // // // // // // //     let timestamp = 300; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // // // // // // // //     let mut context = v_frame::frame::Context::from_file(video_path).unwrap();
// // // // // // // // // // // // // // // // // //     let video_stream = context.streams().best(v_frame::media::Type::Video).unwrap();
// // // // // // // // // // // // // // // // // //     let video_stream_index = video_stream.index();

// // // // // // // // // // // // // // // // // //     let video_info = video_stream.codec_params();
// // // // // // // // // // // // // // // // // //     let width = video_info.width();
// // // // // // // // // // // // // // // // // //     let height = video_info.height();

// // // // // // // // // // // // // // // // // //     context
// // // // // // // // // // // // // // // // // //         .seek(
// // // // // // // // // // // // // // // // // //             timestamp as i64 * video_info.time_base().denominator as i64
// // // // // // // // // // // // // // // // // //                 / video_info.time_base().numerator as i64,
// // // // // // // // // // // // // // // // // //             v_frame::format::context::SeekFrom::Backward,
// // // // // // // // // // // // // // // // // //             video_stream_index,
// // // // // // // // // // // // // // // // // //         )
// // // // // // // // // // // // // // // // // //         .unwrap();

// // // // // // // // // // // // // // // // // //     let mut decoder = video_stream.codec().decoder().video().unwrap();
// // // // // // // // // // // // // // // // // //     let mut frame = Frame::new_default(width, height, PixelType::Rgb8);

// // // // // // // // // // // // // // // // // //     for (stream, packet) in context.packets() {
// // // // // // // // // // // // // // // // // //         if stream.index() == video_stream_index {
// // // // // // // // // // // // // // // // // //             decoder.send_packet(&packet).unwrap();
// // // // // // // // // // // // // // // // // //             if decoder.receive_frame(&mut frame).is_ok() {
// // // // // // // // // // // // // // // // // //                 break;
// // // // // // // // // // // // // // // // // //             }
// // // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // // //     }

// // // // // // // // // // // // // // // // // //     let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // // // // // // // //     frame.write_jpeg(&mut file).unwrap();
// // // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // // // // // // use std::io::BufWriter;
// // // // // // // // // // // // // // // // // use v_frame::frame::Frame;
// // // // // // // // // // // // // // // // // use v_frame::pixel::Rgb8;
// // // // // // // // // // // // // // // // // use v_frame::prelude::*;

// // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // // // // // // //     // let thumbnail_path = "path/to/save/thumbnail.jpg";
// // // // // // // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // // // // // // //     let timestamp = 300; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // // // // // // //     let mut context = v_frame::format::context::Input::from_file(video_path).unwrap();
// // // // // // // // // // // // // // // // //     let video_stream = context.streams().best(v_frame::media::Type::Video).unwrap();
// // // // // // // // // // // // // // // // //     let video_stream_index = video_stream.index();

// // // // // // // // // // // // // // // // //     let video_info = video_stream.codec_params();
// // // // // // // // // // // // // // // // //     let width = video_info.width();
// // // // // // // // // // // // // // // // //     let height = video_info.height();

// // // // // // // // // // // // // // // // //     context
// // // // // // // // // // // // // // // // //         .seek(v_frame::format::context::SeekFrom::Timestamp {
// // // // // // // // // // // // // // // // //             stream_index: video_stream_index,
// // // // // // // // // // // // // // // // //             timestamp: timestamp as i64 * video_info.time_base().denominator as i64
// // // // // // // // // // // // // // // // //                 / video_info.time_base().numerator as i64,
// // // // // // // // // // // // // // // // //             flags: v_frame::format::context::SeekFlags::BACKWARD,
// // // // // // // // // // // // // // // // //         })
// // // // // // // // // // // // // // // // //         .unwrap();

// // // // // // // // // // // // // // // // //     let mut decoder = video_stream.codec().decoder().video().unwrap();
// // // // // // // // // // // // // // // // //     let mut frame = Frame::new(width, height, Rgb8::default());

// // // // // // // // // // // // // // // // //     for (stream, packet) in context.packets() {
// // // // // // // // // // // // // // // // //         if stream.index() == video_stream_index {
// // // // // // // // // // // // // // // // //             decoder.send_packet(&packet).unwrap();
// // // // // // // // // // // // // // // // //             if decoder.receive_frame(&mut frame).is_ok() {
// // // // // // // // // // // // // // // // //                 break;
// // // // // // // // // // // // // // // // //             }
// // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // //     }

// // // // // // // // // // // // // // // // //     let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // // // // // // //     frame.write_jpeg(&mut file).unwrap();
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // // // // // use std::io::BufWriter;
// // // // // // // // // // // // // // // // use std::io::SeekFrom;
// // // // // // // // // // // // // // // // use v_frame::frame::Frame;
// // // // // // // // // // // // // // // // use v_frame::pixel::YUV420;
// // // // // // // // // // // // // // // // use v_frame::prelude::*;

// // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // // // // // //     // let thumbnail_path = "path/to/save/thumbnail.jpg";
// // // // // // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // // // // // //     let timestamp = 300; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // // // // // //     let mut context = v_frame::codec::Context::from_file(video_path).unwrap();
// // // // // // // // // // // // // // // //     let video_stream = context
// // // // // // // // // // // // // // // //         .streams()
// // // // // // // // // // // // // // // //         .best(v_frame::codec::MediaKind::Video)
// // // // // // // // // // // // // // // //         .unwrap();
// // // // // // // // // // // // // // // //     let video_stream_index = video_stream.index();

// // // // // // // // // // // // // // // //     let video_info = video_stream.codec_params();
// // // // // // // // // // // // // // // //     let width = video_info.width();
// // // // // // // // // // // // // // // //     let height = video_info.height();

// // // // // // // // // // // // // // // //     context
// // // // // // // // // // // // // // // //         .seek(
// // // // // // // // // // // // // // // //             SeekFrom::Current(
// // // // // // // // // // // // // // // //                 timestamp as i64 * video_info.time_base().denominator as i64
// // // // // // // // // // // // // // // //                     / video_info.time_base().numerator as i64,
// // // // // // // // // // // // // // // //             ),
// // // // // // // // // // // // // // // //             v_frame::codec::SeekFlag::BACKWARD,
// // // // // // // // // // // // // // // //             video_stream_index,
// // // // // // // // // // // // // // // //         )
// // // // // // // // // // // // // // // //         .unwrap();

// // // // // // // // // // // // // // // //     let mut decoder = video_stream.codec().decoder().video().unwrap();
// // // // // // // // // // // // // // // //     let mut frame = Frame::new_with_padding(width, height, YUV420, 0);

// // // // // // // // // // // // // // // //     for (stream, packet) in context.packets() {
// // // // // // // // // // // // // // // //         if stream.index() == video_stream_index {
// // // // // // // // // // // // // // // //             decoder.send_packet(&packet).unwrap();
// // // // // // // // // // // // // // // //             if decoder.receive_frame(&mut frame).is_ok() {
// // // // // // // // // // // // // // // //                 break;
// // // // // // // // // // // // // // // //             }
// // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // //     }

// // // // // // // // // // // // // // // //     let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // // // // // //     frame.write_jpeg(&mut file).unwrap();
// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // // // // use std::io::BufWriter;
// // // // // // // // // // // // // // // use std::io::SeekFrom;
// // // // // // // // // // // // // // // use v_frame::frame::Frame;
// // // // // // // // // // // // // // // use v_frame::pixel::{ChromaSampling, Pixel};
// // // // // // // // // // // // // // // use v_frame::prelude::*;

// // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // // // // //     // let thumbnail_path = "path/to/save/thumbnail.jpg";
// // // // // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // // // // //     let timestamp = 300; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // // // // //     let mut context = v_frame::format::context::Input::from_file(video_path).unwrap();
// // // // // // // // // // // // // // //     let video_stream = context.streams().best(v_frame::media::Type::Video).unwrap();
// // // // // // // // // // // // // // //     let video_stream_index = video_stream.index();

// // // // // // // // // // // // // // //     let video_info = video_stream.parameters();
// // // // // // // // // // // // // // //     let width = video_info.width();
// // // // // // // // // // // // // // //     let height = video_info.height();

// // // // // // // // // // // // // // //     context
// // // // // // // // // // // // // // //         .seek(
// // // // // // // // // // // // // // //             SeekFrom::Start(
// // // // // // // // // // // // // // //                 timestamp as u64 * video_info.time_base().denominator as u64
// // // // // // // // // // // // // // //                     / video_info.time_base().numerator as u64,
// // // // // // // // // // // // // // //             ),
// // // // // // // // // // // // // // //             ..video_stream_index,
// // // // // // // // // // // // // // //         )
// // // // // // // // // // // // // // //         .unwrap();

// // // // // // // // // // // // // // //     let mut decoder = video_stream.codec().decoder().video().unwrap();
// // // // // // // // // // // // // // //     let mut frame = Frame::new_with_padding(width, height, ChromaSampling::Cs420, 0);

// // // // // // // // // // // // // // //     for (stream, packet) in context.packets() {
// // // // // // // // // // // // // // //         if stream.index() == video_stream_index {
// // // // // // // // // // // // // // //             decoder.send_packet(&packet).unwrap();
// // // // // // // // // // // // // // //             if decoder.receive_frame(&mut frame).is_ok() {
// // // // // // // // // // // // // // //                 break;
// // // // // // // // // // // // // // //             }
// // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // //     }

// // // // // // // // // // // // // // //     let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // // // // //     frame.write_to(&mut file, Frame::jpeg()).unwrap();
// // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // // // use std::io::BufWriter;
// // // // // // // // // // // // // // use std::io::SeekFrom;
// // // // // // // // // // // // // // use v_frame::frame::Frame;
// // // // // // // // // // // // // // use v_frame::pixel::ChromaSampling;

// // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // // // //     // let thumbnail_path = "path/to/save/thumbnail.jpg";
// // // // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // // // //     let timestamp = 300; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // // // //     let mut context = v_frame::context::input::open(video_path).unwrap();
// // // // // // // // // // // // // //     let video_stream = context
// // // // // // // // // // // // // //         .streams()
// // // // // // // // // // // // // //         .best(v_frame::stream::Type::Video)
// // // // // // // // // // // // // //         .unwrap();
// // // // // // // // // // // // // //     let video_stream_index = video_stream.index();

// // // // // // // // // // // // // //     let video_info = video_stream.parameters();
// // // // // // // // // // // // // //     let width = video_info.width();
// // // // // // // // // // // // // //     let height = video_info.height();

// // // // // // // // // // // // // //     context
// // // // // // // // // // // // // //         .seek(
// // // // // // // // // // // // // //             SeekFrom::Start(
// // // // // // // // // // // // // //                 timestamp as u64 * video_info.time_base().denominator as u64
// // // // // // // // // // // // // //                     / video_info.time_base().numerator as u64,
// // // // // // // // // // // // // //             ),
// // // // // // // // // // // // // //             video_stream_index,
// // // // // // // // // // // // // //         )
// // // // // // // // // // // // // //         .unwrap();

// // // // // // // // // // // // // //     let mut decoder = video_stream.codec().decoder().video().unwrap();
// // // // // // // // // // // // // //     let mut frame = Frame::new_with_padding(width, height, ChromaSampling::Cs420, 0);

// // // // // // // // // // // // // //     for (stream, packet) in context.packets() {
// // // // // // // // // // // // // //         if stream.index() == video_stream_index {
// // // // // // // // // // // // // //             decoder.send_packet(&packet).unwrap();
// // // // // // // // // // // // // //             if decoder.receive_frame(&mut frame).is_ok() {
// // // // // // // // // // // // // //                 break;
// // // // // // // // // // // // // //             }
// // // // // // // // // // // // // //         }
// // // // // // // // // // // // // //     }

// // // // // // // // // // // // // //     let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // // // //     frame
// // // // // // // // // // // // // //         .write_to(&mut file, v_frame::encoder::image::jpeg::Jpeg {})
// // // // // // // // // // // // // //         .unwrap();
// // // // // // // // // // // // // // }

// // // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // // use std::io::BufWriter;
// // // // // // // // // // // // // use v_frame::decoder::Decoder;
// // // // // // // // // // // // // use v_frame::encoder::jpeg::JpegEncoder;
// // // // // // // // // // // // // use v_frame::frame::Frame;
// // // // // // // // // // // // // use v_frame::pixel::{ChromaSampling, Formaton};

// // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // // //     // let thumbnail_path = "path/to/save/thumbnail.jpg";
// // // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // // //     let timestamp = 300.0; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // // //     let mut decoder = Decoder::open(&video_path).unwrap();
// // // // // // // // // // // // //     let video_stream = decoder.default_stream().unwrap();
// // // // // // // // // // // // //     let video_info = video_stream.info();

// // // // // // // // // // // // //     let width = video_info.width;
// // // // // // // // // // // // //     let height = video_info.height;

// // // // // // // // // // // // //     decoder.seek(timestamp).unwrap();

// // // // // // // // // // // // //     let mut frame = Frame::new_with_padding(width, height, ChromaSampling::Cs420, 0);

// // // // // // // // // // // // //     while let Some((stream, packet)) = decoder.read().unwrap() {
// // // // // // // // // // // // //         if stream.index() == video_stream.index() {
// // // // // // // // // // // // //             decoder.decode(&packet, &mut frame).unwrap();
// // // // // // // // // // // // //             break;
// // // // // // // // // // // // //         }
// // // // // // // // // // // // //     }

// // // // // // // // // // // // //     let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // // //     let mut encoder = JpegEncoder::new(&mut file);
// // // // // // // // // // // // //     encoder.encode(&frame).unwrap();
// // // // // // // // // // // // // }

// // // // // // // // // // // // use av_codec::decoder::Decoder;
// // // // // // // // // // // // use av_data::frame::ArcFrame;
// // // // // // // // // // // // use av_format::buffer::AccReader;
// // // // // // // // // // // // use av_format::demuxer::{Demuxer, Event};
// // // // // // // // // // // // use av_vorbis::decoder::VORBIS_DESCR;
// // // // // // // // // // // // use libopus::decoder::OPUS_DESCR;
// // // // // // // // // // // // use libvpx::decoder::VP9_DESCR;
// // // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // // use std::io::BufWriter;

// // // // // // // // // // // // fn main() {
// // // // // // // // // // // //     // let video_path = "path/to/your/video.mp4";
// // // // // // // // // // // //     // let thumbnail_path = "path/to/save/thumbnail.jpg";
// // // // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // // // //     let timestamp = 300.0; // 选择视频中的某个时间点作为缩略图(以秒为单位)

// // // // // // // // // // // //     // 打开视频文件
// // // // // // // // // // // //     let input_file = File::open(video_path).unwrap();
// // // // // // // // // // // //     let reader = AccReader::new(input_file);

// // // // // // // // // // // //     // 创建解复用器
// // // // // // // // // // // //     let demuxer = Demuxer::new(reader);

// // // // // // // // // // // //     // 创建视频解码器
// // // // // // // // // // // //     let decoders = [VP9_DESCR, OPUS_DESCR, VORBIS_DESCR];
// // // // // // // // // // // //     let mut decoder = Decoder::from_descriptions(&decoders, &demuxer).unwrap();

// // // // // // // // // // // //     // 定位到指定的时间戳
// // // // // // // // // // // //     demuxer.seek(timestamp).unwrap();

// // // // // // // // // // // //     // 读取视频帧
// // // // // // // // // // // //     let mut frame: Option<ArcFrame> = None;
// // // // // // // // // // // //     while let Ok(event) = demuxer.read_event() {
// // // // // // // // // // // //         match event {
// // // // // // // // // // // //             Event::NewPacket(pkt) => {
// // // // // // // // // // // //                 decoder.send_packet(&pkt).unwrap();
// // // // // // // // // // // //                 if let Ok(f) = decoder.receive_frame() {
// // // // // // // // // // // //                     frame = Some(f);
// // // // // // // // // // // //                     break;
// // // // // // // // // // // //                 }
// // // // // // // // // // // //             }
// // // // // // // // // // // //             Event::Eof => break,
// // // // // // // // // // // //             _ => (),
// // // // // // // // // // // //         }
// // // // // // // // // // // //     }

// // // // // // // // // // // //     // 将视频帧编码为 JPEG 格式并保存
// // // // // // // // // // // //     if let Some(frame) = frame {
// // // // // // // // // // // //         let mut file = BufWriter::new(File::create(thumbnail_path).unwrap());
// // // // // // // // // // // //         let mut encoder = image::jpeg::JpegEncoder::new_with_quality(&mut file, 90);
// // // // // // // // // // // //         let img = image::DynamicImage::ImageRgb8(frame.plane::<[u8; 3]>(0).to_rgb());
// // // // // // // // // // // //         encoder.encode_image(&img).unwrap();
// // // // // // // // // // // //     }
// // // // // // // // // // // // }

// // // // // // // // // // // use std::collections::HashMap;
// // // // // // // // // // // use std::fs::File;
// // // // // // // // // // // use std::path::PathBuf;

// // // // // // // // // // // use clap::Parser;

// // // // // // // // // // // use av_codec::common::CodecList;
// // // // // // // // // // // use av_codec::decoder::{Codecs as DecCodecs, Context as DecContext};
// // // // // // // // // // // use av_data::frame::{ArcFrame, MediaKind};
// // // // // // // // // // // use av_format::buffer::{AccReader, Buffered};
// // // // // // // // // // // use av_format::demuxer::{Context, Demuxer, Event};

// // // // // // // // // // // use matroska::demuxer::MkvDemuxer;

// // // // // // // // // // // use av_vorbis::decoder::VORBIS_DESCR;
// // // // // // // // // // // use libopus::decoder::OPUS_DESCR;
// // // // // // // // // // // use libvpx::decoder::VP9_DESCR;

// // // // // // // // // // // use image::ImageFormat;
// // // // // // // // // // // use std::io::BufWriter;

// // // // // // // // // // // fn decode_single_frame<D: Demuxer, R: Buffered>(
// // // // // // // // // // //     demuxer: &mut Context<D, R>,
// // // // // // // // // // //     decoders: &mut HashMap<isize, DecContext>,
// // // // // // // // // // //     timestamp: f64,
// // // // // // // // // // // ) -> Result<Option<ArcFrame>, String> {
// // // // // // // // // // //     demuxer.seek(timestamp).map_err(|e| e.to_string())?;

// // // // // // // // // // //     loop {
// // // // // // // // // // //         match demuxer.read_event() {
// // // // // // // // // // //             Ok(event) => match event {
// // // // // // // // // // //                 Event::NewPacket(pkt) => {
// // // // // // // // // // //                     if let Some(decoder) = decoders.get_mut(&pkt.stream_index) {
// // // // // // // // // // //                         decoder.send_packet(&pkt).unwrap();
// // // // // // // // // // //                         if let Ok(frame) = decoder.receive_frame() {
// // // // // // // // // // //                             return Ok(Some(frame));
// // // // // // // // // // //                         }
// // // // // // // // // // //                     }
// // // // // // // // // // //                 }
// // // // // // // // // // //                 Event::Eof => return Ok(None),
// // // // // // // // // // //                 _ => (),
// // // // // // // // // // //             },
// // // // // // // // // // //             Err(err) => return Err(err.to_string()),
// // // // // // // // // // //         }
// // // // // // // // // // //     }
// // // // // // // // // // // }

// // // // // // // // // // // #[derive(Parser, Debug)]
// // // // // // // // // // // #[clap(
// // // // // // // // // // //     name = "thumbnail-generator",
// // // // // // // // // // //     version,
// // // // // // // // // // //     author,
// // // // // // // // // // //     about = "Generates a thumbnail from a video file"
// // // // // // // // // // // )]
// // // // // // // // // // // struct Opts {
// // // // // // // // // // //     /// Sets the video file to generate thumbnail from
// // // // // // // // // // //     #[clap(long, short, value_parser)]
// // // // // // // // // // //     input: PathBuf,

// // // // // // // // // // //     /// Sets the output path for the generated thumbnail
// // // // // // // // // // //     #[clap(long, short, value_parser)]
// // // // // // // // // // //     output: PathBuf,

// // // // // // // // // // //     /// Sets the timestamp (in seconds) of the frame to use as thumbnail
// // // // // // // // // // //     #[clap(long, short, value_parser, default_value = "0.0")]
// // // // // // // // // // //     timestamp: f64,
// // // // // // // // // // // }

// // // // // // // // // // // fn main() {
// // // // // // // // // // //     let opts = Opts::parse();

// // // // // // // // // // //     let reader = File::open(opts.input).unwrap();
// // // // // // // // // // //     let ar = AccReader::with_capacity(4 * 1024, reader);
// // // // // // // // // // //     let mut demuxer = Context::new(MkvDemuxer::new(), ar);
// // // // // // // // // // //     demuxer
// // // // // // // // // // //         .read_headers()
// // // // // // // // // // //         .expect("Cannot parse the format headers");

// // // // // // // // // // //     let decoders = DecCodecs::from_list(&[VP9_DESCR, OPUS_DESCR, VORBIS_DESCR]);
// // // // // // // // // // //     let mut decs: HashMap<isize, DecContext> = HashMap::with_capacity(2);

// // // // // // // // // // //     for stream in &demuxer.info.streams {
// // // // // // // // // // //         if let Some(ref codec_id) = stream.params.codec_id {
// // // // // // // // // // //             if let Some(mut ctx) = DecContext::by_name(&decoders, codec_id) {
// // // // // // // // // // //                 if let Some(ref extradata) = stream.params.extradata {
// // // // // // // // // // //                     ctx.set_extradata(extradata);
// // // // // // // // // // //                 }
// // // // // // // // // // //                 ctx.configure().expect("Codec configure failed");
// // // // // // // // // // //                 decs.insert(stream.index as isize, ctx);
// // // // // // // // // // //             }
// // // // // // // // // // //         }
// // // // // // // // // // //     }

// // // // // // // // // // //     if let Ok(Some(frame)) = decode_single_frame(&mut demuxer, &mut decs, opts.timestamp) {
// // // // // // // // // // //         if let MediaKind::Video(_) = frame.kind {
// // // // // // // // // // //             let thumbnail = frame.plane::<[u8; 3]>(0).to_rgb();
// // // // // // // // // // //             let mut output_file = BufWriter::new(File::create(opts.output).unwrap());
// // // // // // // // // // //             thumbnail
// // // // // // // // // // //                 .write_to(&mut output_file, ImageFormat::Jpeg)
// // // // // // // // // // //                 .unwrap();
// // // // // // // // // // //             println!("Thumbnail generated successfully");
// // // // // // // // // // //         } else {
// // // // // // // // // // //             eprintln!(
// // // // // // // // // // //                 "The frame at timestamp {} is not a video frame",
// // // // // // // // // // //                 opts.timestamp
// // // // // // // // // // //             );
// // // // // // // // // // //         }
// // // // // // // // // // //     } else {
// // // // // // // // // // //         eprintln!("Failed to decode frame at timestamp {}", opts.timestamp);
// // // // // // // // // // //     }
// // // // // // // // // // // }

// // // // // // // // // // use std::fs::File;
// // // // // // // // // // use std::io::BufWriter;
// // // // // // // // // // use std::path::Path;
// // // // // // // // // // use std::time::Duration;

// // // // // // // // // // use symphonia::core::codecs::DecoderOptions;
// // // // // // // // // // use symphonia::core::errors::Error;
// // // // // // // // // // use symphonia::core::formats::FormatOptions;
// // // // // // // // // // use symphonia::core::io::MediaSourceStream;
// // // // // // // // // // use symphonia::core::meta::MetadataOptions;
// // // // // // // // // // use symphonia::core::probe::Hint;

// // // // // // // // // // use image::{ImageFormat, ImageResult};

// // // // // // // // // // fn generate_thumbnail<P: AsRef<Path>>(
// // // // // // // // // //     input_path: P,
// // // // // // // // // //     output_path: P,
// // // // // // // // // //     timestamp: Duration,
// // // // // // // // // // ) -> Result<(), Error> {
// // // // // // // // // //     let src = File::open(input_path)?;
// // // // // // // // // //     let mss = MediaSourceStream::new(Box::new(src), Default::default());

// // // // // // // // // //     let mut hint = Hint::new();
// // // // // // // // // //     hint.with_extension("mkv");

// // // // // // // // // //     let format_opts: FormatOptions = Default::default();
// // // // // // // // // //     let metadata_opts: MetadataOptions = Default::default();
// // // // // // // // // //     let decoder_opts: DecoderOptions = Default::default();

// // // // // // // // // //     let probed = symphonia::default::get_probe()
// // // // // // // // // //         .format(&hint, mss, &format_opts, &metadata_opts)
// // // // // // // // // //         .unwrap();

// // // // // // // // // //     let mut format = probed.format;
// // // // // // // // // //     let track = format
// // // // // // // // // //         .tracks()
// // // // // // // // // //         .iter()
// // // // // // // // // //         .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
// // // // // // // // // //         .unwrap();

// // // // // // // // // //     let mut decoder = symphonia::default::get_codecs()
// // // // // // // // // //         .make(&track.codec_params, &decoder_opts)
// // // // // // // // // //         .unwrap();

// // // // // // // // // //     format.seek(
// // // // // // // // // //         track.id,
// // // // // // // // // //         timestamp,
// // // // // // // // // //         symphonia::core::formats::SeekMode::Accurate,
// // // // // // // // // //     )?;

// // // // // // // // // //     let mut frame_count = 0;
// // // // // // // // // //     while let Ok(packet) = format.next_packet() {
// // // // // // // // // //         if packet.track_id() != track.id {
// // // // // // // // // //             continue;
// // // // // // // // // //         }

// // // // // // // // // //         decoder.decode(&packet)?;
// // // // // // // // // //         frame_count += 1;

// // // // // // // // // //         if decoder.codec_params().time_base.is_none() {
// // // // // // // // // //             continue;
// // // // // // // // // //         }

// // // // // // // // // //         let time_base = decoder.codec_params().time_base.unwrap();
// // // // // // // // // //         let ts = (packet.ts() as f64) * time_base;

// // // // // // // // // //         if ts >= timestamp.as_secs_f64() {
// // // // // // // // // //             break;
// // // // // // // // // //         }
// // // // // // // // // //     }

// // // // // // // // // //     let frame = decoder.retrieve_frame(frame_count - 1)?;
// // // // // // // // // //     let width = frame.width() as u32;
// // // // // // // // // //     let height = frame.height() as u32;

// // // // // // // // // //     let thumbnail = image::ImageBuffer::from_raw(width, height, frame.data(0).to_vec()).unwrap();

// // // // // // // // // //     let mut output_file = BufWriter::new(File::create(output_path)?);
// // // // // // // // // //     let write_result: ImageResult<()> = thumbnail.write_to(&mut output_file, ImageFormat::Jpeg);
// // // // // // // // // //     write_result?;

// // // // // // // // // //     Ok(())
// // // // // // // // // // }

// // // // // // // // // // fn main() {
// // // // // // // // // //     // let input_path = "path/to/your/video.mkv";
// // // // // // // // // //     // let output_path = "path/to/thumbnail.jpg";
// // // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // // //     let timestamp = Duration::from_secs(300);

// // // // // // // // // //     match generate_thumbnail(input_path, output_path, timestamp) {
// // // // // // // // // //         Ok(_) => println!("Thumbnail generated successfully"),
// // // // // // // // // //         Err(e) => eprintln!("Failed to generate thumbnail: {}", e),
// // // // // // // // // //     }
// // // // // // // // // // }

// // // // // // // // // use std::fs::File;
// // // // // // // // // use std::io::BufWriter;
// // // // // // // // // use std::path::Path;
// // // // // // // // // use std::time::Duration;

// // // // // // // // // use symphonia::core::audio::SampleBuffer;
// // // // // // // // // use symphonia::core::codecs::DecoderOptions;
// // // // // // // // // use symphonia::core::errors::Error;
// // // // // // // // // use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
// // // // // // // // // use symphonia::core::io::MediaSourceStream;
// // // // // // // // // use symphonia::core::meta::MetadataOptions;
// // // // // // // // // use symphonia::core::probe::Hint;

// // // // // // // // // use image::{ImageBuffer, ImageFormat, Rgb};

// // // // // // // // // fn generate_thumbnail<P: AsRef<Path>>(
// // // // // // // // //     input_path: P,
// // // // // // // // //     output_path: P,
// // // // // // // // //     timestamp: Duration,
// // // // // // // // // ) -> Result<(), Error> {
// // // // // // // // //     let src = File::open(input_path)?;
// // // // // // // // //     let mss = MediaSourceStream::new(Box::new(src), Default::default());

// // // // // // // // //     let mut hint = Hint::new();
// // // // // // // // //     hint.with_extension("mkv");

// // // // // // // // //     let format_opts: FormatOptions = Default::default();
// // // // // // // // //     let metadata_opts: MetadataOptions = Default::default();
// // // // // // // // //     let decoder_opts: DecoderOptions = Default::default();

// // // // // // // // //     let probed = symphonia::default::get_probe()
// // // // // // // // //         .format(&hint, mss, &format_opts, &metadata_opts)
// // // // // // // // //         .unwrap();

// // // // // // // // //     let mut format = probed.format;
// // // // // // // // //     let track = format
// // // // // // // // //         .tracks()
// // // // // // // // //         .iter()
// // // // // // // // //         .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
// // // // // // // // //         .unwrap();

// // // // // // // // //     let mut decoder = symphonia::default::get_codecs()
// // // // // // // // //         .make(&track.codec_params, &decoder_opts)
// // // // // // // // //         .unwrap();

// // // // // // // // //     let time_base = track.codec_params.time_base.unwrap();
// // // // // // // // //     let seek_ts = timestamp.as_secs_f64() / time_base.as_secs_f64();

// // // // // // // // //     format.seek(
// // // // // // // // //         SeekMode::Accurate,
// // // // // // // // //         SeekTo::Time {
// // // // // // // // //             time: seek_ts as u64,
// // // // // // // // //             track_id: Some(track.id),
// // // // // // // // //         },
// // // // // // // // //     )?;

// // // // // // // // //     let mut frame_count = 0;
// // // // // // // // //     while let Ok(packet) = format.next_packet() {
// // // // // // // // //         if packet.track_id() != track.id {
// // // // // // // // //             continue;
// // // // // // // // //         }

// // // // // // // // //         decoder.decode(&packet)?;

// // // // // // // // //         if let Some(audio_buf) = decoder.audio_buffer() {
// // // // // // // // //             frame_count += 1;

// // // // // // // // //             if frame_count == 1 {
// // // // // // // // //                 let spec = *audio_buf.spec();
// // // // // // // // //                 let duration =
// // // // // // // // //                     audio_buf.capacity() as u64 * time_base.denominator / time_base.numerator;

// // // // // // // // //                 if duration >= (timestamp.as_secs_f64() * spec.rate as f64 / 1000.0) as u64 {
// // // // // // // // //                     break;
// // // // // // // // //                 }
// // // // // // // // //             }
// // // // // // // // //         }
// // // // // // // // //     }

// // // // // // // // //     let audio_buf = decoder.audio_buffer().unwrap();
// // // // // // // // //     let samples = audio_buf.samples();

// // // // // // // // //     let thumbnail_width = 256;
// // // // // // // // //     let thumbnail_height = 256;
// // // // // // // // //     let mut thumbnail_data = Vec::with_capacity(thumbnail_width * thumbnail_height * 3);

// // // // // // // // //     for y in 0..thumbnail_height {
// // // // // // // // //         for x in 0..thumbnail_width {
// // // // // // // // //             let sample_idx = (samples.len() as u64 * (x + y * thumbnail_width) as u64
// // // // // // // // //                 / (thumbnail_width * thumbnail_height) as u64)
// // // // // // // // //                 as usize;
// // // // // // // // //             let sample = samples[sample_idx];

// // // // // // // // //             let r = ((sample.to_float() + 1.0) * 127.5) as u8;
// // // // // // // // //             let g = ((sample.to_float() + 1.0) * 127.5) as u8;
// // // // // // // // //             let b = ((sample.to_float() + 1.0) * 127.5) as u8;

// // // // // // // // //             thumbnail_data.extend_from_slice(&[r, g, b]);
// // // // // // // // //         }
// // // // // // // // //     }

// // // // // // // // //     let thumbnail = ImageBuffer::from_vec(
// // // // // // // // //         thumbnail_width as u32,
// // // // // // // // //         thumbnail_height as u32,
// // // // // // // // //         thumbnail_data,
// // // // // // // // //     )
// // // // // // // // //     .unwrap();
// // // // // // // // //     let thumbnail = image::DynamicImage::ImageRgb8(thumbnail);

// // // // // // // // //     let mut output_file = BufWriter::new(File::create(output_path)?);
// // // // // // // // //     thumbnail.write_to(&mut output_file, ImageFormat::Jpeg)?;

// // // // // // // // //     Ok(())
// // // // // // // // // }

// // // // // // // // // fn main() {
// // // // // // // // //     // let input_path = "path/to/your/video.mkv";
// // // // // // // // //     // let output_path = "path/to/thumbnail.jpg";
// // // // // // // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // // //     let thumbnail_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // // //     let timestamp = Duration::from_secs(300);

// // // // // // // // //     match generate_thumbnail(input_path, output_path, timestamp) {
// // // // // // // // //         Ok(_) => println!("Thumbnail generated successfully"),
// // // // // // // // //         Err(e) => eprintln!("Failed to generate thumbnail: {}", e),
// // // // // // // // //     }
// // // // // // // // // }

// // // // // // // // use std::fs::File;
// // // // // // // // use std::io::BufWriter;
// // // // // // // // use std::path::Path;
// // // // // // // // use std::time::Duration;

// // // // // // // // use symphonia::core::codecs::DecoderOptions;
// // // // // // // // use symphonia::core::errors::Error;
// // // // // // // // use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
// // // // // // // // use symphonia::core::io::MediaSourceStream;
// // // // // // // // use symphonia::core::meta::MetadataOptions;
// // // // // // // // use symphonia::core::probe::Hint;
// // // // // // // // use symphonia::core::units::Time;

// // // // // // // // use image::{ImageBuffer, ImageFormat};

// // // // // // // // fn generate_thumbnail<P: AsRef<Path>>(
// // // // // // // //     input_path: P,
// // // // // // // //     output_path: P,
// // // // // // // //     timestamp: Duration,
// // // // // // // // ) -> Result<(), Box<dyn std::error::Error>> {
// // // // // // // //     let src = File::open(input_path)?;
// // // // // // // //     let mss = MediaSourceStream::new(Box::new(src), Default::default());

// // // // // // // //     let mut hint = Hint::new();
// // // // // // // //     hint.with_extension("mkv");

// // // // // // // //     let format_opts: FormatOptions = Default::default();
// // // // // // // //     let metadata_opts: MetadataOptions = Default::default();
// // // // // // // //     let decoder_opts: DecoderOptions = Default::default();

// // // // // // // //     let probed = symphonia::default::get_probe()
// // // // // // // //         .format(&hint, mss, &format_opts, &metadata_opts)
// // // // // // // //         .unwrap();

// // // // // // // //     let mut format = probed.format;
// // // // // // // //     let track = format
// // // // // // // //         .tracks()
// // // // // // // //         .iter()
// // // // // // // //         .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
// // // // // // // //         .unwrap();

// // // // // // // //     let mut decoder = symphonia::default::get_codecs()
// // // // // // // //         .make(&track.codec_params, &decoder_opts)
// // // // // // // //         .unwrap();

// // // // // // // //     let time_base = track.codec_params.time_base.unwrap();
// // // // // // // //     let seek_ts = timestamp.as_secs() / time_base.numer as u64 * time_base.denom as u64;

// // // // // // // //     format.seek(
// // // // // // // //         SeekMode::Accurate,
// // // // // // // //         SeekTo::Time {
// // // // // // // //             time: Time::from(seek_ts),
// // // // // // // //             track_id: Some(track.id),
// // // // // // // //         },
// // // // // // // //     )?;

// // // // // // // //     let mut frame_count = 0;
// // // // // // // //     while let Ok(packet) = format.next_packet() {
// // // // // // // //         if packet.track_id() != track.id {
// // // // // // // //             continue;
// // // // // // // //         }

// // // // // // // //         decoder.decode(&packet)?;

// // // // // // // //         if let Some(audio_buf) = decoder.audio_buffer.clone() {
// // // // // // // //             frame_count += 1;

// // // // // // // //             if frame_count == 1 {
// // // // // // // //                 let spec = *audio_buf.spec();
// // // // // // // //                 let duration =
// // // // // // // //                     audio_buf.capacity() as u64 * time_base.denom as u64 / time_base.numer as u64;

// // // // // // // //                 if duration >= (timestamp.as_secs_f64() * spec.rate as f64 / 1000.0) as u64 {
// // // // // // // //                     break;
// // // // // // // //                 }
// // // // // // // //             }
// // // // // // // //         }
// // // // // // // //     }

// // // // // // // //     let audio_buf = decoder.audio_buffer.clone().unwrap();
// // // // // // // //     let samples = audio_buf.samples();

// // // // // // // //     let thumbnail_width = 256;
// // // // // // // //     let thumbnail_height = 256;
// // // // // // // //     let mut thumbnail_data = Vec::with_capacity(thumbnail_width * thumbnail_height * 3);

// // // // // // // //     for y in 0..thumbnail_height {
// // // // // // // //         for x in 0..thumbnail_width {
// // // // // // // //             let sample_idx = (samples.len() as u64 * (x + y * thumbnail_width) as u64
// // // // // // // //                 / (thumbnail_width * thumbnail_height) as u64)
// // // // // // // //                 as usize;
// // // // // // // //             let sample = samples[sample_idx];

// // // // // // // //             let r = ((sample.to_float() + 1.0) * 127.5) as u8;
// // // // // // // //             let g = ((sample.to_float() + 1.0) * 127.5) as u8;
// // // // // // // //             let b = ((sample.to_float() + 1.0) * 127.5) as u8;

// // // // // // // //             thumbnail_data.extend_from_slice(&[r, g, b]);
// // // // // // // //         }
// // // // // // // //     }

// // // // // // // //     let thumbnail = ImageBuffer::from_vec(
// // // // // // // //         thumbnail_width as u32,
// // // // // // // //         thumbnail_height as u32,
// // // // // // // //         thumbnail_data,
// // // // // // // //     )
// // // // // // // //     .unwrap();
// // // // // // // //     let thumbnail = image::DynamicImage::ImageRgb8(thumbnail);

// // // // // // // //     let mut output_file = BufWriter::new(File::create(output_path)?);
// // // // // // // //     thumbnail
// // // // // // // //         .write_to(&mut output_file, ImageFormat::Jpeg)
// // // // // // // //         .map_err(|e| e as Box<dyn std::error::Error>)?;

// // // // // // // //     Ok(())
// // // // // // // // }

// // // // // // // // fn main() {
// // // // // // // //     // let input_path = "path/to/your/video.mkv";
// // // // // // // //     // let output_path = "path/to/thumbnail.jpg";
// // // // // // // //     let input_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // // //     let output_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // // //     let timestamp = Duration::from_secs(300);

// // // // // // // //     match generate_thumbnail(input_path, output_path, timestamp) {
// // // // // // // //         Ok(_) => println!("Thumbnail generated successfully"),
// // // // // // // //         Err(e) => eprintln!("Failed to generate thumbnail: {}", e),
// // // // // // // //     }
// // // // // // // // }

// // // // // // // use std::fs::File;
// // // // // // // use std::io::BufWriter;
// // // // // // // use std::path::Path;
// // // // // // // use std::time::Duration;

// // // // // // // use symphonia::core::codecs::DecoderOptions;
// // // // // // // use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
// // // // // // // use symphonia::core::io::MediaSourceStream;
// // // // // // // use symphonia::core::meta::MetadataOptions;
// // // // // // // use symphonia::core::probe::Hint;
// // // // // // // use symphonia::core::units::Time;

// // // // // // // use image::{ImageBuffer, ImageFormat};

// // // // // // // fn generate_thumbnail<P: AsRef<Path>>(
// // // // // // //     input_path: P,
// // // // // // //     output_path: P,
// // // // // // //     timestamp: Duration,
// // // // // // // ) -> Result<(), Box<dyn std::error::Error>> {
// // // // // // //     let src = File::open(input_path)?;
// // // // // // //     let mss = MediaSourceStream::new(Box::new(src), Default::default());

// // // // // // //     let mut hint = Hint::new();
// // // // // // //     hint.with_extension("mkv");

// // // // // // //     let format_opts: FormatOptions = Default::default();
// // // // // // //     let metadata_opts: MetadataOptions = Default::default();
// // // // // // //     let decoder_opts: DecoderOptions = Default::default();

// // // // // // //     let probed = symphonia::default::get_probe()
// // // // // // //         .format(&hint, mss, &format_opts, &metadata_opts)
// // // // // // //         .unwrap();

// // // // // // //     let mut format = probed.format;
// // // // // // //     let track = format
// // // // // // //         .tracks()
// // // // // // //         .iter()
// // // // // // //         .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
// // // // // // //         .unwrap();

// // // // // // //     let mut decoder = symphonia::default::get_codecs()
// // // // // // //         .make(&track.codec_params, &decoder_opts)
// // // // // // //         .unwrap();

// // // // // // //     let time_base = track.codec_params.time_base.unwrap();
// // // // // // //     let seek_ts = timestamp.as_secs() / time_base.numer as u64 * time_base.denom as u64;

// // // // // // //     format.seek(
// // // // // // //         SeekMode::Accurate,
// // // // // // //         SeekTo::Time {
// // // // // // //             time: Time::from(seek_ts),
// // // // // // //             track_id: Some(track.id),
// // // // // // //         },
// // // // // // //     )?;

// // // // // // //     let mut frame_count = 0;
// // // // // // //     while let Ok(packet) = format.next_packet() {
// // // // // // //         if packet.track_id() != track.id {
// // // // // // //             continue;
// // // // // // //         }

// // // // // // //         decoder.decode(&packet)?;

// // // // // // //         if let Some(audio_buf) = decoder
// // // // // // //             .codec_params()
// // // // // // //             .audio
// // // // // // //             .as_ref()
// // // // // // //             .and_then(|a| a.buffer.clone())
// // // // // // //         {
// // // // // // //             frame_count += 1;

// // // // // // //             if frame_count == 1 {
// // // // // // //                 let spec = *audio_buf.spec();
// // // // // // //                 let duration =
// // // // // // //                     audio_buf.capacity() as u64 * time_base.denom as u64 / time_base.numer as u64;

// // // // // // //                 if duration >= (timestamp.as_secs_f64() * spec.rate as f64 / 1000.0) as u64 {
// // // // // // //                     break;
// // // // // // //                 }
// // // // // // //             }
// // // // // // //         }
// // // // // // //     }

// // // // // // //     let audio_buf = decoder
// // // // // // //         .codec_params()
// // // // // // //         .audio
// // // // // // //         .as_ref()
// // // // // // //         .and_then(|a| a.buffer.clone())
// // // // // // //         .unwrap();
// // // // // // //     let samples = audio_buf.samples();

// // // // // // //     let thumbnail_width = 256;
// // // // // // //     let thumbnail_height = 256;
// // // // // // //     let mut thumbnail_data = Vec::with_capacity(thumbnail_width * thumbnail_height * 3);

// // // // // // //     for y in 0..thumbnail_height {
// // // // // // //         for x in 0..thumbnail_width {
// // // // // // //             let sample_idx = (samples.len() as u64 * (x + y * thumbnail_width) as u64
// // // // // // //                 / (thumbnail_width * thumbnail_height) as u64)
// // // // // // //                 as usize;
// // // // // // //             let sample = samples[sample_idx];

// // // // // // //             let r = ((sample.to_float() + 1.0) * 127.5) as u8;
// // // // // // //             let g = ((sample.to_float() + 1.0) * 127.5) as u8;
// // // // // // //             let b = ((sample.to_float() + 1.0) * 127.5) as u8;

// // // // // // //             thumbnail_data.extend_from_slice(&[r, g, b]);
// // // // // // //         }
// // // // // // //     }

// // // // // // //     let thumbnail = ImageBuffer::from_vec(
// // // // // // //         thumbnail_width as u32,
// // // // // // //         thumbnail_height as u32,
// // // // // // //         thumbnail_data,
// // // // // // //     )
// // // // // // //     .unwrap();
// // // // // // //     let thumbnail = image::DynamicImage::ImageRgb8(thumbnail);

// // // // // // //     let mut output_file = BufWriter::new(File::create(output_path)?);
// // // // // // //     thumbnail
// // // // // // //         .write_to(&mut output_file, ImageFormat::Jpeg)
// // // // // // //         .map_err(|e| e.into())?;

// // // // // // //     Ok(())
// // // // // // // }

// // // // // // // fn main() {
// // // // // // //     // let input_path = "path/to/your/video.mkv";
// // // // // // //     // let output_path = "path/to/thumbnail.jpg";
// // // // // // //     let input_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // // //     let output_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // // //     let timestamp = Duration::from_secs(300);

// // // // // // //     match generate_thumbnail(input_path, output_path, timestamp) {
// // // // // // //         Ok(_) => println!("Thumbnail generated successfully"),
// // // // // // //         Err(e) => eprintln!("Failed to generate thumbnail: {}", e),
// // // // // // //     }
// // // // // // // }

// // // // // // use std::fs::File;
// // // // // // use std::io::BufWriter;
// // // // // // use std::path::Path;
// // // // // // use std::time::Duration;

// // // // // // use symphonia::core::codecs::DecoderOptions;
// // // // // // use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
// // // // // // use symphonia::core::io::MediaSourceStream;
// // // // // // use symphonia::core::meta::MetadataOptions;
// // // // // // use symphonia::core::probe::Hint;
// // // // // // use symphonia::core::units::Time;

// // // // // // use image::{ImageBuffer, ImageFormat};

// // // // // // fn generate_thumbnail<P: AsRef<Path>>(
// // // // // //     input_path: P,
// // // // // //     output_path: P,
// // // // // //     timestamp: Duration,
// // // // // // ) -> Result<(), Box<dyn std::error::Error>> {
// // // // // //     let src = File::open(input_path)?;
// // // // // //     let mss = MediaSourceStream::new(Box::new(src), Default::default());

// // // // // //     let mut hint = Hint::new();
// // // // // //     hint.with_extension("mkv");

// // // // // //     let format_opts: FormatOptions = Default::default();
// // // // // //     let metadata_opts: MetadataOptions = Default::default();
// // // // // //     let decoder_opts: DecoderOptions = Default::default();

// // // // // //     let probed = symphonia::default::get_probe()
// // // // // //         .format(&hint, mss, &format_opts, &metadata_opts)
// // // // // //         .unwrap();

// // // // // //     let mut format = probed.format;
// // // // // //     let track = format
// // // // // //         .tracks()
// // // // // //         .iter()
// // // // // //         .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
// // // // // //         .unwrap();

// // // // // //     let mut decoder = symphonia::default::get_codecs()
// // // // // //         .make(&track.codec_params, &decoder_opts)
// // // // // //         .unwrap();

// // // // // //     let time_base = track.codec_params.time_base.unwrap();
// // // // // //     let seek_ts = timestamp.as_secs() / time_base.numer as u64 * time_base.denom as u64;

// // // // // //     format.seek(
// // // // // //         SeekMode::Accurate,
// // // // // //         SeekTo::Time {
// // // // // //             time: Time::from(seek_ts),
// // // // // //             track_id: Some(track.id),
// // // // // //         },
// // // // // //     )?;

// // // // // //     let mut frame_count = 0;
// // // // // //     while let Ok(packet) = format.next_packet() {
// // // // // //         if packet.track_id() != track.id {
// // // // // //             continue;
// // // // // //         }

// // // // // //         decoder.decode(&packet)?;

// // // // // //         if let Some(audio_buf) = decoder
// // // // // //             .codec_params()
// // // // // //             .sample_format
// // // // // //             .as_ref()
// // // // // //             .map(|f| f.buffer::<f32>())
// // // // // //             .unwrap_or_default()
// // // // // //         {
// // // // // //             frame_count += 1;

// // // // // //             if frame_count == 1 {
// // // // // //                 let spec = *audio_buf.spec();
// // // // // //                 let duration =
// // // // // //                     audio_buf.capacity() as u64 * time_base.denom as u64 / time_base.numer as u64;

// // // // // //                 if duration >= (timestamp.as_secs_f64() * spec.rate as f64 / 1000.0) as u64 {
// // // // // //                     break;
// // // // // //                 }
// // // // // //             }
// // // // // //         }
// // // // // //     }

// // // // // //     let audio_buf = decoder
// // // // // //         .codec_params()
// // // // // //         .sample_format
// // // // // //         .as_ref()
// // // // // //         .map(|f| f.buffer::<f32>())
// // // // // //         .unwrap_or_default();
// // // // // //     let samples = audio_buf.into_samples();

// // // // // //     let thumbnail_width = 256;
// // // // // //     let thumbnail_height = 256;
// // // // // //     let mut thumbnail_data = Vec::with_capacity(thumbnail_width * thumbnail_height * 3);

// // // // // //     for y in 0..thumbnail_height {
// // // // // //         for x in 0..thumbnail_width {
// // // // // //             let sample_idx = (samples.len() as u64 * (x + y * thumbnail_width) as u64
// // // // // //                 / (thumbnail_width * thumbnail_height) as u64)
// // // // // //                 as usize;
// // // // // //             let sample = samples[sample_idx];

// // // // // //             let r = ((sample + 1.0) * 127.5) as u8;
// // // // // //             let g = ((sample + 1.0) * 127.5) as u8;
// // // // // //             let b = ((sample + 1.0) * 127.5) as u8;

// // // // // //             thumbnail_data.extend_from_slice(&[r, g, b]);
// // // // // //         }
// // // // // //     }

// // // // // //     let thumbnail = ImageBuffer::from_vec(
// // // // // //         thumbnail_width as u32,
// // // // // //         thumbnail_height as u32,
// // // // // //         thumbnail_data,
// // // // // //     )
// // // // // //     .unwrap();
// // // // // //     let thumbnail = image::DynamicImage::ImageRgb8(thumbnail);

// // // // // //     let mut output_file = BufWriter::new(File::create(output_path)?);
// // // // // //     thumbnail
// // // // // //         .write_to(&mut output_file, ImageFormat::Jpeg)
// // // // // //         .map_err(|e| e.into())?;

// // // // // //     Ok(())
// // // // // // }

// // // // // // fn main() {
// // // // // //     // let input_path = "path/to/your/video.mkv";
// // // // // //     // let output_path = "path/to/thumbnail.jpg";
// // // // // //     let input_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // // //     let output_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // // //     let timestamp = Duration::from_secs(300);

// // // // // //     match generate_thumbnail(input_path, output_path, timestamp) {
// // // // // //         Ok(_) => println!("Thumbnail generated successfully"),
// // // // // //         Err(e) => eprintln!("Failed to generate thumbnail: {}", e),
// // // // // //     }
// // // // // // }

// // // // // use std::fs::File;
// // // // // use std::io::BufWriter;
// // // // // use std::path::Path;
// // // // // use std::time::Duration;

// // // // // use symphonia::core::codecs::DecoderOptions;
// // // // // use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo};
// // // // // use symphonia::core::io::MediaSourceStream;
// // // // // use symphonia::core::meta::MetadataOptions;
// // // // // use symphonia::core::probe::Hint;
// // // // // use symphonia::core::units::Time;

// // // // // use image::{ImageBuffer, ImageFormat};

// // // // // fn generate_thumbnail<P: AsRef<Path>>(
// // // // //     input_path: P,
// // // // //     output_path: P,
// // // // //     timestamp: Duration,
// // // // // ) -> Result<(), Box<dyn std::error::Error>> {
// // // // //     let src = File::open(input_path)?;
// // // // //     let mss = MediaSourceStream::new(Box::new(src), Default::default());

// // // // //     let mut hint = Hint::new();
// // // // //     hint.with_extension("mkv");

// // // // //     let format_opts: FormatOptions = Default::default();
// // // // //     let metadata_opts: MetadataOptions = Default::default();
// // // // //     let decoder_opts: DecoderOptions = Default::default();

// // // // //     let probed = symphonia::default::get_probe()
// // // // //         .format(&hint, mss, &format_opts, &metadata_opts)
// // // // //         .unwrap();

// // // // //     let mut format = probed.format;
// // // // //     let track = format
// // // // //         .tracks()
// // // // //         .iter()
// // // // //         .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
// // // // //         .unwrap();

// // // // //     let mut decoder = symphonia::default::get_codecs()
// // // // //         .make(&track.codec_params, &decoder_opts)
// // // // //         .unwrap();

// // // // //     let time_base = track.codec_params.time_base.unwrap();
// // // // //     let seek_ts = timestamp.as_secs() / time_base.numer as u64 * time_base.denom as u64;

// // // // //     format.seek(
// // // // //         SeekMode::Accurate,
// // // // //         SeekTo::Time {
// // // // //             time: Time::from(seek_ts),
// // // // //             track_id: Some(track.id),
// // // // //         },
// // // // //     )?;

// // // // //     let mut frame_count = 0;
// // // // //     while let Ok(packet) = format.next_packet() {
// // // // //         if packet.track_id() != track.id {
// // // // //             continue;
// // // // //         }

// // // // //         decoder.decode(&packet)?;

// // // // //         if let Some(audio_buf) = decoder
// // // // //             .codec_params()
// // // // //             .sample_format
// // // // //             .as_ref()
// // // // //             .and_then(|f| f.new_interleaved_buffer::<f32>())
// // // // //         {
// // // // //             frame_count += 1;

// // // // //             if frame_count == 1 {
// // // // //                 let spec = *audio_buf.spec();
// // // // //                 let duration =
// // // // //                     audio_buf.capacity() as u64 * time_base.denom as u64 / time_base.numer as u64;

// // // // //                 if duration >= (timestamp.as_secs_f64() * spec.rate as f64 / 1000.0) as u64 {
// // // // //                     break;
// // // // //                 }
// // // // //             }
// // // // //         }
// // // // //     }

// // // // //     let audio_buf = decoder
// // // // //         .codec_params()
// // // // //         .sample_format
// // // // //         .as_ref()
// // // // //         .and_then(|f| f.new_interleaved_buffer::<f32>())
// // // // //         .unwrap();
// // // // //     let samples = audio_buf.data();

// // // // //     let thumbnail_width = 256;
// // // // //     let thumbnail_height = 256;
// // // // //     let mut thumbnail_data = Vec::with_capacity(thumbnail_width * thumbnail_height * 3);

// // // // //     for y in 0..thumbnail_height {
// // // // //         for x in 0..thumbnail_width {
// // // // //             let sample_idx = (samples.len() as u64 * (x + y * thumbnail_width) as u64
// // // // //                 / (thumbnail_width * thumbnail_height) as u64)
// // // // //                 as usize;
// // // // //             let sample = samples[sample_idx];

// // // // //             let r = ((sample + 1.0) * 127.5) as u8;
// // // // //             let g = ((sample + 1.0) * 127.5) as u8;
// // // // //             let b = ((sample + 1.0) * 127.5) as u8;

// // // // //             thumbnail_data.extend_from_slice(&[r, g, b]);
// // // // //         }
// // // // //     }

// // // // //     let thumbnail = ImageBuffer::from_vec(
// // // // //         thumbnail_width as u32,
// // // // //         thumbnail_height as u32,
// // // // //         thumbnail_data,
// // // // //     )
// // // // //     .unwrap();
// // // // //     let thumbnail = image::DynamicImage::ImageRgb8(thumbnail);

// // // // //     let mut output_file = BufWriter::new(File::create(output_path)?);
// // // // //     thumbnail
// // // // //         .write_to(&mut output_file, ImageFormat::Jpeg)
// // // // //         .map_err(|e| e.into())?;

// // // // //     Ok(())
// // // // // }

// // // // // fn main() {
// // // // //     // let input_path = "path/to/your/video.mkv";
// // // // //     // let output_path = "path/to/thumbnail.jpg";
// // // // //     let input_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // // //     let output_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";
// // // // //     let timestamp = Duration::from_secs(300);

// // // // //     match generate_thumbnail(input_path, output_path, timestamp) {
// // // // //         Ok(_) => println!("Thumbnail generated successfully"),
// // // // //         Err(e) => eprintln!("Failed to generate thumbnail: {}", e),
// // // // //     }
// // // // // }

// // // // use std::process::Command;

// // // // fn generate_thumbnails(
// // // //     video_path: &str,
// // // //     output_pattern: &str,
// // // //     timestamps: &[f32],
// // // // ) -> Result<(), Box<dyn std::error::Error>> {
// // // //     let mut command = Command::new("ffmpeg");
// // // //     command.args(&["-i", video_path, "-vf"]);

// // // //     let thumbnail_filter = timestamps
// // // //         .iter()
// // // //         .map(|&t| format!("select='gte(t,{})',scale=320:240,snapshot", t))
// // // //         .collect::<Vec<_>>()
// // // //         .join(";");

// // // //     command.arg(thumbnail_filter);

// // // //     for (i, _) in timestamps.iter().enumerate() {
// // // //         let output_path = format!("{}{}.jpg", output_pattern, i);
// // // //         command.arg(output_path);
// // // //     }

// // // //     let output = command.output()?;

// // // //     if output.status.success() {
// // // //         Ok(())
// // // //     } else {
// // // //         Err(format!("FFmpeg command failed: {:?}", output).into())
// // // //     }
// // // // }

// // // // fn main() {
// // // //     //     let input_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // //     //     let output_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.jpg";

// // // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // // //     let output_pattern = "thumbnail_";
// // // //     let timestamps = &[5.0, 10.0, 15.0, 20.0, 25.0];

// // // //     match generate_thumbnails(video_path, output_pattern, timestamps) {
// // // //         Ok(_) => println!("Thumbnails generated successfully"),
// // // //         Err(e) => eprintln!("Failed to generate thumbnails: {}", e),
// // // //     }
// // // // }

// // // use std::process::Command;

// // // fn generate_thumbnails(
// // //     video_path: &str,
// // //     output_path: &str,
// // //     timestamps: &[f32],
// // //     grid_size: (usize, usize),
// // // ) -> Result<(), Box<dyn std::error::Error>> {
// // //     let mut command = Command::new("ffmpeg");
// // //     command.args(&["-i", video_path, "-vf"]);

// // //     let thumbnail_filter = timestamps
// // //         .iter()
// // //         .map(|&t| format!("select='gte(t,{})',scale=320:240", t))
// // //         .collect::<Vec<_>>()
// // //         .join(";");

// // //     let grid_filter = format!("tile={}x{}", grid_size.0, grid_size.1);
// // //     let filter_complex = format!("{};{}", thumbnail_filter, grid_filter);

// // //     command.arg(filter_complex);
// // //     command.args(&["-frames:v", "1", output_path]);

// // //     let output = command.output()?;

// // //     if output.status.success() {
// // //         Ok(())
// // //     } else {
// // //         Err(format!("FFmpeg command failed: {:?}", output).into())
// // //     }
// // // }

// // // fn main() {
// // //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// // //     let output_path = "thumbnails.jpg";
// // //     let timestamps = &[5.0, 10.0, 15.0, 20.0, 25.0];
// // //     let grid_size = (3, 2);

// // //     match generate_thumbnails(video_path, output_path, timestamps, grid_size) {
// // //         Ok(_) => println!("Thumbnails generated successfully"),
// // //         Err(e) => eprintln!("Failed to generate thumbnails: {}", e),
// // //     }
// // // }

// // use std::process::Command;

// // fn generate_thumbnails(
// //     video_path: &str,
// //     output_path: &str,
// //     timestamps: &[f32],
// //     grid_size: (usize, usize),
// // ) -> Result<(), Box<dyn std::error::Error>> {
// //     let mut command = Command::new("ffmpeg");
// //     command.args(&["-i", video_path, "-vf"]);

// //     let thumbnail_filters = timestamps
// //         .iter()
// //         .map(|&t| {
// //             format!(
// //                 "[0:v]select='gte(t,{})',scale=320:240[thumb{}];",
// //                 t,
// //                 timestamps.iter().position(|&ts| ts == t).unwrap()
// //             )
// //         })
// //         .collect::<Vec<_>>()
// //         .join("");

// //     let mut tile_inputs = Vec::new();
// //     for i in 0..timestamps.len() {
// //         tile_inputs.push(format!("[thumb{}]", i));
// //     }
// //     let tile_filter = format!(
// //         "{}hstack=inputs={}[grid];[grid]scale={}:{}[thumbnail]",
// //         tile_inputs.join(""),
// //         grid_size.0,
// //         grid_size.0 * 320,
// //         grid_size.1 * 240
// //     );

// //     let filter_complex = format!("{}{}", thumbnail_filters, tile_filter);

// //     command.arg(filter_complex);
// //     command.args(&["-map", "[thumbnail]", "-frames:v", "1", output_path]);

// //     let output = command.output()?;

// //     if output.status.success() {
// //         Ok(())
// //     } else {
// //         Err(format!("FFmpeg command failed: {:?}", output).into())
// //     }
// // }

// // fn main() {
// //     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
// //     let output_path = "thumbnails.jpg";
// //     let timestamps = &[5.0, 10.0, 15.0, 20.0, 25.0];
// //     let grid_size = (3, 2);

// //     match generate_thumbnails(video_path, output_path, timestamps, grid_size) {
// //         Ok(_) => println!("Thumbnails generated successfully"),
// //         Err(e) => eprintln!("Failed to generate thumbnails: {}", e),
// //     }
// // }

// use std::process::Command;

// fn generate_thumbnails(
//     video_path: &str,
//     output_path: &str,
//     timestamps: &[f32],
//     grid_size: (usize, usize),
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut command = Command::new("ffmpeg");
//     command.args(&["-i", video_path, "-vf"]);

//     let thumbnail_filters = timestamps
//         .iter()
//         .map(|&t| {
//             format!(
//                 "[0:v]select='gte(t,{})',scale=320:240[thumb{}]",
//                 t,
//                 timestamps.iter().position(|&ts| ts == t).unwrap()
//             )
//         })
//         .collect::<Vec<_>>()
//         .join(";");

//     let mut tile_inputs = Vec::new();
//     for i in 0..timestamps.len() {
//         tile_inputs.push(format!("[thumb{}]", i));
//     }

//     let tile_filter = format!(
//         "{}hstack=inputs={}[grid];[grid]scale={}:{}[thumbnail]",
//         tile_inputs.join(""),
//         grid_size.0,
//         grid_size.0 * 320,
//         grid_size.1 * 240
//     );

//     let filter_complex = format!("{};{}", thumbnail_filters, tile_filter);

//     command.arg("-filter_complex").arg(filter_complex);
//     command.args(&["-map", "[thumbnail]", "-frames:v", "1", output_path]);

//     let output = command.output()?;

//     if output.status.success() {
//         Ok(())
//     } else {
//         Err(format!("FFmpeg command failed: {:?}", output).into())
//     }
// }

// fn main() {
//     let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
//     let output_path = "thumbnails.jpg";
//     let timestamps = &[5.0, 10.0, 15.0, 20.0, 25.0];
//     let grid_size = (3, 2);

//     match generate_thumbnails(video_path, output_path, timestamps, grid_size) {
//         Ok(_) => println!("Thumbnails generated successfully"),
//         Err(e) => eprintln!("Failed to generate thumbnails: {}", e),
//     }
// }

use std::process::Command;

fn generate_thumbnails(
    video_path: &str,
    output_path: &str,
    timestamps: &[f32],
    grid_size: (usize, usize),
) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("ffmpeg");
    command.args(&["-i", video_path]);

    let thumbnail_filters = timestamps
        .iter()
        .map(|&t| {
            format!(
                "[0:v]select='gte(t,{})',scale=320:240[thumb{}]",
                t,
                timestamps.iter().position(|&ts| ts == t).unwrap()
            )
        })
        .collect::<Vec<_>>()
        .join(";");

    let mut tile_inputs = Vec::new();
    for i in 0..timestamps.len() {
        tile_inputs.push(format!("[thumb{}]", i));
    }

    let tile_filter = format!(
        "{}hstack=inputs={}[grid];[grid]scale={}:{}[thumbnail]",
        tile_inputs.join(""),
        grid_size.0,
        grid_size.0 * 320,
        grid_size.1 * 240
    );

    let filter_complex = format!("{};{}", thumbnail_filters, tile_filter);

    command.args(&["-filter_complex", &filter_complex]);
    command.args(&["-map", "[thumbnail]", "-frames:v", "1", output_path]);

    let output = command.output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("FFmpeg command failed: {:?}", output).into())
    }
}

fn main() {
    let video_path = "../../../../../douyinCut/@魏老板私服/@魏老板私服_2024-03-18_07-33-23.mp4";
    let output_path = "thumbnails.jpg";
    let timestamps = &[5.0, 10.0, 15.0, 20.0, 25.0];
    let grid_size = (3, 2);

    match generate_thumbnails(video_path, output_path, timestamps, grid_size) {
        Ok(_) => println!("Thumbnails generated successfully"),
        Err(e) => eprintln!("Failed to generate thumbnails: {}", e),
    }
}
