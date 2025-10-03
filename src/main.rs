use reqwest::blocking::Client;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let client = Client::new();

    if args[1] == "create" {
        let url = &args[2];
        let res: serde_json::Value = client
            .post("https:/shtly.ddns.net/api/shorten")
            .json(&serde_json::json!({ "originalUrl": url }))
            .send()
            .unwrap()
            .json()
            .unwrap();
        println!("Shortened URL: {}", res["shortUrl"]);
    }
}
