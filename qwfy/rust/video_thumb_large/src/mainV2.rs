use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const THUMBNAIL_INTERVAL_SECONDS: i32 = 30;

trait ThumbnailGenerator {
    fn generate(&self, file_path: &str, output_pattern: &str) -> Result<(), String>;
}

struct FFmpegThumbnailGenerator;

impl ThumbnailGenerator for FFmpegThumbnailGenerator {
    fn generate(&self, file_path: &str, output_pattern: &str) -> Result<(), String> {
        let status = Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(file_path)
            .arg("-vf")
            .arg(format!("fps=1/{}", THUMBNAIL_INTERVAL_SECONDS))
            .arg("-vsync")
            .arg("vfr")
            .arg(output_pattern)
            .status()
            .map_err(|_| "Failed to execute ffmpeg".to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err("Failed to generate thumbnails".into())
        }
    }
}

trait FileRenamer {
    fn rename(&self, input_dir: &Path, video_filename: &str) -> std::io::Result<()>;
}

struct DefaultFileRenamer;

impl FileRenamer for DefaultFileRenamer {
    fn rename(&self, input_dir: &Path, video_filename: &str) -> std::io::Result<()> {
        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if let Some(frame_number_str) = filename
                        .strip_prefix(video_filename)
                        .and_then(|f| f.strip_suffix(".jpg"))
                        .and_then(|f| f.trim_start_matches('_').parse::<i32>().ok())
                    {
                        let seconds = frame_number_str * THUMBNAIL_INTERVAL_SECONDS;
                        let new_filename =
                            format!("{}_{}.jpg", video_filename, seconds_to_timestamp(seconds));
                        let new_path = input_dir.join(new_filename);
                        fs::rename(path, new_path)?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn seconds_to_timestamp(seconds: i32) -> String {
    format!(
        "{:02}_{:02}_{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}

struct VideoProcessorBuilder {
    base_path: PathBuf,
    thumbnail_generator: Box<dyn ThumbnailGenerator>,
    file_renamer: Box<dyn FileRenamer>,
}

impl VideoProcessorBuilder {
    fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            thumbnail_generator: Box::new(FFmpegThumbnailGenerator),
            file_renamer: Box::new(DefaultFileRenamer),
        }
    }

    fn with_thumbnail_generator<T: ThumbnailGenerator + 'static>(mut self, generator: T) -> Self {
        self.thumbnail_generator = Box::new(generator);
        self
    }

    fn with_file_renamer<T: FileRenamer + 'static>(mut self, renamer: T) -> Self {
        self.file_renamer = Box::new(renamer);
        self
    }

    fn build(self) -> VideoProcessor {
        VideoProcessor {
            base_path: self.base_path,
            thumbnail_generator: self.thumbnail_generator,
            file_renamer: self.file_renamer,
        }
    }
}

struct VideoProcessor {
    base_path: PathBuf,
    thumbnail_generator: Box<dyn ThumbnailGenerator>,
    file_renamer: Box<dyn FileRenamer>,
}

impl VideoProcessor {
    fn process_videos(&self) -> std::io::Result<()> {
        let video_ext = "mp4";
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map(|s| s == video_ext).unwrap_or(false) {
                self.process_video(&path)?;
            }
        }
        Ok(())
    }

    fn process_video(&self, video_path: &Path) -> std::io::Result<()> {
        let video_name = video_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let output_dir = self.base_path.join(format!("{}_thumb", video_name));
        let output_pattern = output_dir.join(format!("{}_%03d.jpg", video_name));

        self.ensure_directory_exists(&output_dir)?;

        if let Err(e) = self.thumbnail_generator.generate(
            video_path.to_str().unwrap(),
            output_pattern.to_str().unwrap(),
        ) {
            eprintln!("Error generating thumbnails for '{}': {}", video_name, e);
            return Ok(());
        }

        if let Err(e) = self.file_renamer.rename(&output_dir, video_name) {
            eprintln!("Error renaming files for '{}': {}", video_name, e);
            return Ok(());
        }

        println!("Thumbnails generated successfully for '{}'", video_name);
        Ok(())
    }

    fn ensure_directory_exists(&self, path: &Path) -> std::io::Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }
}

fn main() {
    // let base_path = "../../../downloads/抖音直播/诗篇女装旗舰店";
    let base_path = "../../../../../douyinCut/@魏老板私服";
    let processor = VideoProcessorBuilder::new(PathBuf::from(base_path)).build();

    if let Err(e) = processor.process_videos() {
        eprintln!("Failed to process videos in '{}': {}", base_path, e);
    }
}
