
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
            &format!("{}@{}:{}", server_user, server_host, server_path),
        ])
        .status()
        .expect("Failed to execute sshpass command");

    if status.success() {
        println!("Thumbnails synced to server successfully");
    } else {
        eprintln!("Failed to sync thumbnails to server");
    }
}