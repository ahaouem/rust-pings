use reqwest;
use std::sync::{Arc, Mutex};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    println!("dziala");

    let json_data = Arc::new(Mutex::new(serde_json::json!({
        "uuid": "9aaefbd3-8154-4893-9d46-fd20910a55b3",
    })));

    let tasks: Vec<_> = (0..1500)
        .map(|_| {
            let json_data: Arc<Mutex<_>> = Arc::clone(&json_data);
            tokio::spawn(async move {
                make_request(json_data).await;
            })
        })
        .collect();

    for task in tasks {
        task.await.expect("Task panicked");
    }
}

async fn make_request(json_data: Arc<Mutex<serde_json::Value>>) {
    loop {
        let client: reqwest::Client = reqwest::Client::new();
        let data: serde_json::Value = json_data.lock().unwrap().clone();
        let response: Result<reqwest::Response, reqwest::Error> = client
            .post("https://wpis.dumnizpowstancow.pl/upvote")
            .json(&data)
            .send()
            .await;

        if let Ok(n) = response {
            if n.status() == 200 {
                if let Ok(m) = n.text().await {
                    println!(
                        "{:?}",
                        m.split(",").collect::<Vec<&str>>()[0]
                            .split(":")
                            .collect::<Vec<&str>>()[1]
                    );
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}