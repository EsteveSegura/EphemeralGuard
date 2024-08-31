use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use serde_json::{json, Value};
use std::str;

use super::traits::Server;
use crate::db::core::DatabaseCore;
use crate::db::models::credential::Credential;
use crate::log;
use crate::config;

pub struct TcpServer;

impl TcpServer {
    pub fn new() -> Self {
        TcpServer
    }
    
    fn handle_request(db_core: &DatabaseCore, request: &str) -> String {
        let v: Value = match serde_json::from_str(request) {
            Ok(v) => v,
            Err(_) => return json!({"status": "error", "message": "Invalid JSON"}).to_string(),
        };

        match v["action"].as_str() {
            Some("CREATE") => {
                let payload = v["payload"].as_str().unwrap_or("");
                let expiration = v["expiration"].as_u64().unwrap_or(3600); // Default 1 hour
                
                let credential = DatabaseCore::generate_random_credential();
                let credential_string = Credential::get_credential_string(&credential.encryption_iv, &credential.encryption_key);
                
                match db_core.create_secret(payload, expiration, &credential) {
                    Ok(secret) => json!({"status": "success", "id": secret.id, "credential": credential_string}).to_string(),
                    Err(e) => {
                        log!("ERR-TCPSERVER", &format!("Failed to create secret: {}", e));
                        json!({"status": "error", "message": e}).to_string()
                    },
                }
            }
            Some("READ") => {
                let id = v["id"].as_str().unwrap_or("");
                let credential = v["credential"].as_str().unwrap_or("");
            
            
                match Credential::new_from_string(&credential.to_string()) {
                    Ok(credential_from_string) => {
                        match db_core.read_secret(id) {
                            Ok(Some(secret)) => json!({"status": "success", "payload": secret.decrypt(&credential_from_string)}).to_string(),
                            Ok(None) => json!({"status": "error", "message": "Secret not found or expired"}).to_string(),
                            Err(e) => json!({"status": "error", "message": e}).to_string(),
                        }
                    },
                    Err(e) => {
                        log!("ERR-TCPSERVER", &format!("Failed to parse credential: {}", e));
                        json!({"status": "error", "message": format!("Failed to parse credential: {}", e)}).to_string()
                    }
                }
            }
            Some("DELETE") => {
                let id = v["id"].as_str().unwrap_or("");
                match db_core.delete_secret(id) {
                    Ok(true) => json!({"status": "success", "message": "Secret deleted"}).to_string(),
                    Ok(false) => json!({"status": "error", "message": "Secret not found"}).to_string(),
                    Err(e) => json!({"status": "error", "message": e}).to_string(),
                }
            }
            _ => json!({"status": "error", "message": "Invalid action"}).to_string(),
        }
    }

    fn handle_client(db_core: &DatabaseCore, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    let request = str::from_utf8(&buffer[..size]).unwrap_or("");
                    log!("INFO-TCPSERVER", &format!("{}", request));

                    let response = TcpServer::handle_request(db_core, request);

                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
            Err(e) => {
                log!("ERR-TCPSERVER", &format!("Failed to read from stream: {}", e));
            }
        }
    }
}

impl Server for TcpServer {
    fn start(&self, db_core: &DatabaseCore) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", config::TCP_SERVER_PORT)).unwrap();

        log!("INFO", format!("TCP Server running on port {}", config::TCP_SERVER_PORT).as_str());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let db_core = db_core.clone();
                    std::thread::spawn(move || {
                        TcpServer::handle_client(&db_core, stream);
                    });
                }
                Err(e) => {
                    log!("ERR-TCPSERVER", &format!("Connection failed: {}", e));
                }
            }
        }
    }
}
