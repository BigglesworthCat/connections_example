use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct ClientMessage {
    username: String,
    data: String,
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let url = "http://127.0.0.1:12345/send";

    // Ask for a username
    println!("Enter your username: ");
    let mut username = String::new();
    std::io::stdin()
        .read_line(&mut username)
        .expect("Failed to read input");
    let username = username.trim().to_string();

    loop {
        println!("Enter a message to send (or type 'exit' to quit): ");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let message = input.trim();
        if message.eq_ignore_ascii_case("exit") {
            break;
        }

        let msg = ClientMessage {
            username: username.clone(),
            data: message.to_string(),
        };

        // Send http POST request
        match client.post(url).json(&msg).send().await {
            Ok(response) => match response.json::<serde_json::Value>().await {
                Ok(json) => println!("Server response: {}", json),
                Err(e) => eprintln!("Failed to parse response: {}", e),
            },
            Err(e) => eprintln!("Failed to send request: {}", e),
        }
    }
}
