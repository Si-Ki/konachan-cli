use reqwest::StatusCode;
use std::io;

mod utils;

[tokio::main]
async fn main() {
    let client = utils::get_client();
    println!("Tag: ");

    let mut tag = String::new();
    
    io::stdin()
        .read_line(&mut tag);
    
    let mut url = "https://konachan.com/post?tags={}", tag;
    let result = client.get(url).send().await.unwrap();

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong!"),
    };

    println!("HTML: {}", raw_html);
}