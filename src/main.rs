use futures::{stream, StreamExt}; // 0.3.8
use reqwest::Client; // 0.10.9
use tokio; // 0.2.24, features = ["macros"]

const THREAD_NUM: usize = 5;

#[tokio::main]
async fn main() {
    let urls = vec!["https://api.ipify.org"; 100];

    let client = Client::new();

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            tokio::spawn(async move {
                let resp = client.get(url).send().await?;
                resp.text().await
            })
        })
        .buffer_unordered(THREAD_NUM);

    bodies
        .for_each(|b| async {
            match b {
                Ok(Ok(b)) => println!("Your IP : {}", b),
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
}