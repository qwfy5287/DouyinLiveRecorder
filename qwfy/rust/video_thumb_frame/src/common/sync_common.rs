
use std::process::Command;


pub fn sync_thumbnails_to_server(output_root: &str, server_user: &str, server_host: &str, server_path: &str, password: &str) {
    let status = Command::new("sshpass")
        .args(&[
            "-p",
            password,
            "rsync",
            "-avz",
            "--progress",
            "-e",
            "ssh -o StrictHostKeyChecking=no",
            // output_root,
            &(output_root.to_owned()+"/"),
            &format!("{}@{}:{}/", server_user, server_host, server_path),
        ])
        .status()
        .expect("Failed to execute sshpass command");

    if status.success() {
        println!("Thumbnails synced to server successfully");
    } else {
        eprintln!("Failed to sync thumbnails to server");
    }
}

pub fn sync_single_folder(output_dir: &str) {
    let server_user = "root";
    let server_host = "124.70.131.130";
    let server_path = "/var/www/thumb";
    let server_password = "huaweiyundouyinlive@123";

    // 获取 output_dir 的最后 4 个目录
    let output_dir_parts: Vec<&str> = output_dir.split("/").collect();
    // let last_four_parts = &output_dir_parts[output_dir_parts.len() - 4..];
    // let server_dir = format!("{}{}", server_path, last_four_parts.join("/"));

    // output_dir 去掉最后 4 个目录
    let output_root = &output_dir_parts[0..output_dir_parts.len() - 4].join("/");

    // println!("output_dir: {}", output_dir);
    // println!("server_dir: {}", server_dir);
    // println!("output_root: {}", output_root);

    sync_thumbnails_to_server(output_root, server_user, server_host, server_path, server_password);
}