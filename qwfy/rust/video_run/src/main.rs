// use std::process::Command;

// fn run_python_script() {
//     let output = Command::new("python3.11")
//         .arg("main.py")
//         // 如果Python脚本不在当前目录中，你可以使用cwd方法来改变工作目录
//         .current_dir("../../../")
//         .output()
//         .expect("Failed to execute python script");

//     if output.status.success() {
//         println!("Python output as raw bytes: {:?}", output.stdout);

//         let stdout =
//             String::from_utf8(output.stdout).expect("Python script output is not valid UTF-8");
//         println!("Python script output: {}", stdout);

//         // // 打印Python脚本的标准输出
//         // let stdout = String::from_utf8(output.stdout).unwrap();
//         // println!("Python script output: {}", stdout);
//     } else {
//         // 如果有错误，打印错误并退出
//         let stderr = String::from_utf8(output.stderr).unwrap();
//         eprintln!("Python script failed: {}", stderr);
//         std::process::exit(1);
//     }
// }

// fn main() {
//     run_python_script();
// }

use std::process::Command;

fn main() {
    let output = Command::new("python3.11")
        .arg("main.py")
        .current_dir("../../../")
        .output()
        .expect("Failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
