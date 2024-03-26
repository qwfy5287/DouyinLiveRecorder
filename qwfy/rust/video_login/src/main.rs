// main.rs

mod common;

use crate::common::login_common::login;

#[tokio::main]
async fn main() {
    let login_result = login("18250833087".to_string(), "qwfy@123!456".to_string()).await;
    
    match login_result {
        Ok(response) => {
            if response.code == 20000 {
                println!("Hello");
            } else {
                eprintln!("Login failed: {}", response.message);
                println!("login fail");
            }
        }
        Err(err) => {
            eprintln!("Request error: {}", err);
            println!("login fail");
        }
    }
}