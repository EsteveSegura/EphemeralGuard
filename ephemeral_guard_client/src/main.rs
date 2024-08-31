use clap::{Arg, Command};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let matches = Command::new("EphemeralGuard CLI")
        .version("1.0")
        .author("Esteve Segura")
        .about("CLI for connecting to EphemeralGuard")
        .subcommand(
            Command::new("add_secret")
                .about("Adds a new secret")
                .arg(
                    Arg::new("payload")
                        .short('p')
                        .long("payload")
                        .value_name("PAYLOAD")
                        .help("The secret value")
                        .required(true),
                )
                .arg(
                    Arg::new("time")
                        .short('t')
                        .long("time")
                        .value_name("EXPIRATION")
                        .help("Time in seconds until expiration from now")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("read_secret")
                .about("Reads an existing secret")
                .arg(
                    Arg::new("id")
                        .short('i')
                        .long("id")
                        .value_name("ID")
                        .help("The secret ID")
                        .required(true),
                )
                .arg(
                    Arg::new("credentials")
                        .short('c')
                        .long("credentials")
                        .value_name("CREDENTIALS")
                        .help("The credentials for the secret")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("delete_secret")
                .about("Deletes an existing secret")
                .arg(
                    Arg::new("id")
                        .short('i')
                        .long("id")
                        .value_name("ID")
                        .help("The secret ID")
                        .required(true),
                ),
        )
        .get_matches();

    let server_address = "127.0.0.1:1337";
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            match matches.subcommand() {
                Some(("add_secret", sub_matches)) => {
                    let payload = sub_matches.get_one::<String>("payload").unwrap();
                    let expiration: u64 = sub_matches.get_one::<String>("time").unwrap().parse().unwrap();
                    add_secret(&mut stream, payload, expiration);
                }
                Some(("read_secret", sub_matches)) => {
                    let id = sub_matches.get_one::<String>("id").unwrap();
                    let credential = sub_matches.get_one::<String>("credentials").unwrap();
                    read_secret(&mut stream, id, credential);
                }
                Some(("delete_secret", sub_matches)) => {
                    let id = sub_matches.get_one::<String>("id").unwrap();
                    delete_secret(&mut stream, id);
                }
                _ => {
                    eprintln!("Unrecognized command.");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
        }
    }
}

fn add_secret(stream: &mut TcpStream, payload: &str, expiration: u64) {
    let request = json!({
        "action": "CREATE",
        "payload": payload,
        "expiration": expiration,
    });

    send_request(stream, &request.to_string());
}

fn read_secret(stream: &mut TcpStream, id: &str, credential: &str) {
    let request = json!({
        "action": "READ",
        "id": id,
        "credential": credential
    });

    send_request(stream, &request.to_string());
}

fn delete_secret(stream: &mut TcpStream, id: &str) {
    let request = json!({
        "action": "DELETE",
        "id": id
    });

    send_request(stream, &request.to_string());
}

fn send_request(stream: &mut TcpStream, request: &str) {
    stream.write_all(request.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..size]);

    print_response(response.to_string());
}

fn print_response(response: String) {
    let v: Value = serde_json::from_str(&response).unwrap();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}
