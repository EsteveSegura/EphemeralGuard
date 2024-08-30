use zmq;
use serde_json::{json, Value};
use std::io::{self, Write};

fn main() {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("tcp://localhost:5555").is_ok());

    println!("Connected to server");

    loop {
        print_menu();
        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => create_secret(&requester),
            "2" => read_secret(&requester),
            "3" => delete_secret(&requester),
            "4" => create_n_secrets(&requester),
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

fn create_secret(requester: &zmq::Socket) {
    let payload = get_user_input("Enter the secret: ");
    let expiration = get_user_input("Enter expiration time in seconds: ");

    let request = json!({
        "action": "CREATE",
        "payload": payload,
        "expiration": expiration.parse::<u64>().unwrap_or(3600),
    });

    requester.send(&request.to_string(), 0).unwrap();
    let response = requester.recv_string(0).unwrap().unwrap();
    print_response(response);
}

fn read_secret(requester: &zmq::Socket) {
    let id = get_user_input("Enter the secret ID: ");
    let credential = get_user_input("Enter the credentials: ");

    let request = json!({
        "action": "READ",
        "id": id,
        "credential": credential
    });

    requester.send(&request.to_string(), 0).unwrap();
    let response = requester.recv_string(0).unwrap().unwrap();
    print_response(response);
}

fn delete_secret(requester: &zmq::Socket) {
    let id = get_user_input("Enter the secret ID to delete: ");

    let request = json!({
        "action": "DELETE",
        "id": id
    });

    requester.send(&request.to_string(), 0).unwrap();
    let response = requester.recv_string(0).unwrap().unwrap();
    print_response(response);
}

fn create_n_secrets(requester: &zmq::Socket) {
    let number_secrets_to_create:i64 = get_user_input("How many secrets to create: ").parse().unwrap();
    let expiration_secrets:i64 = get_user_input("Expiration in n seconds: ").parse().unwrap();
    
    for i in 0..number_secrets_to_create {
        let payload = format!("Secret number {}", i + 1);
        let request = json!({
            "action": "CREATE",
            "payload": payload,
            "expiration": expiration_secrets,
        });
        
        requester.send(&request.to_string(), 0).unwrap();
        let response = requester.recv_string(0).unwrap().unwrap();
        
        if i < 10 {
            print_response(response);
            println!("Only showing 10 secrets, but {} were created.", number_secrets_to_create);
        }
    }
    println!("Successfully created {} secrets with {} seconds of expiration.", number_secrets_to_create, expiration_secrets);
}

fn print_response(response: String) {
    let v: Value = serde_json::from_str(&response).unwrap();
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
