use reqwest::blocking::Client;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let client = Client::new();
    
    if args.len() < 3 {
        eprintln!("Usage: {} create <url>", args[0]);
        std::process::exit(1);
    }
    
    if args[1] == "create" {
        let url = &args[2];
        let res: serde_json::Value = match client
            .post("https://shtly.ddns.net/api/shorten")
            .json(&serde_json::json!({ "originalUrl": url }))
            .send()
        {
            Ok(response) => match response.json() {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("Failed to parse response: {}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Request failed: {}", e);
                std::process::exit(1);
            }
        };
        if let Some(short_url) = res.get("shortUrl") {
            println!("Shortened URL: {}", short_url);
        } else {
            eprintln!("Unexpected response format: {}", res);
            std::process::exit(1);
        }
    } else {
        eprintln!("Unknown command: {}", args[1]);
        std::process::exit(1);
    }
}
