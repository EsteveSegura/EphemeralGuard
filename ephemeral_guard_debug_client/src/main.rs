use serde_json::{json, Value};
use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    println!("EphemeralGuard Client started.");

    loop {
        print_menu();
        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => create_secret(),
            "2" => read_secret(),
            "3" => delete_secret(),
            "4" => create_n_secrets(),
            "5" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }

    println!("Exiting client.");
}

fn print_menu() {
    println!("\n--- EphemeralGuard Client ---");
    println!("1. Create Secret");
    println!("2. Read Secret");
    println!("3. Delete Secret");
    println!("4. Create n Secrets with x seconds expiration");
    println!("5. Exit");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn create_secret() {
    let payload = get_user_input("Enter the secret: ");
    let expiration = get_user_input("Enter expiration time in seconds: ");

    let request = json!({
        "action": "CREATE",
        "payload": payload,
        "expiration": expiration.parse::<u64>().unwrap_or(3600),
    });

    send_request(request);
}

fn read_secret() {
    let id = get_user_input("Enter the secret ID: ");
    let credential = get_user_input("Enter the credentials: ");

    let request = json!({
        "action": "READ",
        "id": id,
        "credential": credential
    });

    send_request(request);
}

fn delete_secret() {
    let id = get_user_input("Enter the secret ID to delete: ");

    let request = json!({
        "action": "DELETE",
        "id": id
    });

    send_request(request);
}

fn create_n_secrets() {
    let number_secrets_to_create: i64 = get_user_input("How many secrets to create: ").parse().unwrap();
    let expiration_secrets: i64 = get_user_input("Expiration in n seconds: ").parse().unwrap();
    
    for i in 0..number_secrets_to_create {
        let payload = format!("Secret number {}", i + 1);
        let request = json!({
            "action": "CREATE",
            "payload": payload,
            "expiration": expiration_secrets,
        });

        send_request(request);
        
        if i < 10 {
            println!("Created secret {}", i + 1);
            if i == 9 {
                println!("Only showing 10 secrets, but {} were created.", number_secrets_to_create);
            }
        }
    }
    println!("Successfully created {} secrets with {} seconds of expiration.", number_secrets_to_create, expiration_secrets);
}

fn send_request(request: Value) {
    let server_address = "127.0.0.1:1337";
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            stream.write_all(request.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();

            let response = receive_response(&mut stream);
            print_response(response);
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
        }
    }
}

fn receive_response(stream: &mut TcpStream) -> String {
    let mut buffer = vec![0; 4096];
    let size = stream.read(&mut buffer).unwrap_or(0);
    String::from_utf8_lossy(&buffer[..size]).to_string()
}

fn print_response(response: String) {
    if response.is_empty() {
        println!("Received an empty response from the server.");
        return;
    }

    let v: Value = match serde_json::from_str(&response) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Failed to parse response: {}", e);
            return;
        }
    };

    println!("Server response:");
    println!("Status: {}", v["status"].as_str().unwrap());
    if v["status"] == "success" {
        if let Some(id) = v["id"].as_str() {
            println!("ID: {}", id);
        }
        if let Some(payload) = v["payload"].as_str() {
            println!("Payload: {}", payload);
        }
        if let Some(credential) = v["credential"].as_str() {
            println!("Credentials: {}", credential);
        }
        if let Some(message) = v["message"].as_str() {
            println!("Message: {}", message);
        }
    } else if let Some(message) = v["message"].as_str() {
        println!("Error: {}", message);
    }
}
