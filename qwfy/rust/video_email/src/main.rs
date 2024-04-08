use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let email = Message::builder()
        .from("发件人 <qwfy5287@qq.com>".parse().unwrap())
        .to("收件人 <qwfy5287@gmail.com>".parse().unwrap())
        .subject("刘一一_货盘有更新")
        .body(String::from("这是一封使用 Rust lettre 库通过 QQ 邮箱发送的测试邮件"))
        .unwrap();

    let creds = Credentials::new("qwfy5287@qq.com".to_string(), "zcflswqrjtkdbdfc".to_string());

    // QQ 邮箱 SMTP 服务器地址
    let mailer = SmtpTransport::relay("smtp.qq.com")
        .unwrap()
        .credentials(creds)
        .build();

    // 发送邮件
    match mailer.send(&email) {
        Ok(_) => println!("邮件发送成功!"),
        Err(e) => panic!("无法发送邮件: {:?}", e),
    }
}