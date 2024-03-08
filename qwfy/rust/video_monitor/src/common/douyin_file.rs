use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn write_live_link_to_file(live_link: &str) -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("../../../config");
    fs::create_dir_all(&config_path)?;

    let file_path = config_path.join("URL_config.ini");
    let mut content = String::new();

    if file_path.exists() {
        content = fs::read_to_string(&file_path)?;
    }

    if !content.contains(live_link) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;

        if !content.is_empty() {
            writeln!(file, "{}", live_link)?;
        } else {
            write!(file, "{}", live_link)?;
        }
    }

    Ok(())
}

pub fn read_urls_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let buf = io::BufReader::new(file);
    let urls = buf
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.split(',').next().unwrap().trim().to_string())
        .collect();
    Ok(urls)
}
