use clap::{Arg, Command};
use serde_json::{json, Value};
use zmq;

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

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("tcp://localhost:5555").is_ok());

    match matches.subcommand() {
        Some(("add_secret", sub_matches)) => {
            let payload = sub_matches.get_one::<String>("payload").unwrap();
            let expiration: u64 = sub_matches.get_one::<String>("time").unwrap().parse().unwrap();
            add_secret(&requester, payload, expiration);
        }
        Some(("read_secret", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap();
            let credential = sub_matches.get_one::<String>("credentials").unwrap();
            read_secret(&requester, id, credential);
        }
        Some(("delete_secret", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap();
            delete_secret(&requester, id);
        }
        _ => {
            eprintln!("Unrecognized command.");
        }
    }
}

fn add_secret(requester: &zmq::Socket, payload: &str, expiration: u64) {
    let request = json!({
        "action": "CREATE",
        "payload": payload,
        "expiration": expiration,
    });

    requester.send(&request.to_string(), 0).unwrap();
    let response = requester.recv_string(0).unwrap().unwrap();
    print_response(response);
}

fn read_secret(requester: &zmq::Socket, id: &str, credential: &str) {
    let request = json!({
        "action": "READ",
        "id": id,
        "credential": credential
    });

    requester.send(&request.to_string(), 0).unwrap();
    let response = requester.recv_string(0).unwrap().unwrap();
    print_response(response);
}

fn delete_secret(requester: &zmq::Socket, id: &str) {
    let request = json!({
        "action": "DELETE",
        "id": id
    });

    requester.send(&request.to_string(), 0).unwrap();
    let response = requester.recv_string(0).unwrap().unwrap();
    print_response(response);
}

fn print_response(response: String) {
    let v: Value = serde_json::from_str(&response).unwrap();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}
