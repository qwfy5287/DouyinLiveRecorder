// mod common;

// use crate::common::email_common::send_email;

// fn main() {
//     let from = "发件人 <qwfy5287@qq.com>";
//     let to = "收件人 <qwfy5287@gmail.com>";
//     let subject = "刘一一_货盘有更新_01";
//     let body = "这是一封使用 Rust lettre 库通过 QQ 邮箱发送的测试邮件";
//     let smtp_server = "smtp.qq.com";
//     let username = "qwfy5287@qq.com";
//     let password = "zcflswqrjtkdbdfc";

//     match send_email(from, to, subject, body, smtp_server, username, password) {
//         Ok(_) => println!("邮件发送成功!"),
//         Err(e) => eprintln!("无法发送邮件: {:?}", e),
//     }
// }


mod common;

use crate::common::email_common::send_email;

fn main() {
    let from = "发件人 <qwfy5287@qq.com>";
    let to = vec![
        "收件人1 <qwfy5287@gmail.com>",
        "收件人2 <719425597@qq.com>",
    ];
    let subject = "刘一一_货盘有更新";
    let body = "这是一封使用 Rust lettre 库通过 QQ 邮箱发送的测试邮件";
    let smtp_server = "smtp.qq.com";
    let username = "qwfy5287@qq.com";
    let password = "zcflswqrjtkdbdfc";

    match send_email(from, &to, subject, body, smtp_server, username, password) {
        Ok(_) => println!("邮件发送成功!"),
        Err(e) => eprintln!("无法发送邮件: {:?}", e),
    }
}