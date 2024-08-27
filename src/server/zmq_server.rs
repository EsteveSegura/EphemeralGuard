use zmq;
use serde_json::{json, Value};

use super::traits::Server;

use crate::db::core::DatabaseCore;
use crate::db::models::credential::Credential;

use crate::log;

pub struct ZmqServer;

impl ZmqServer {
    pub fn new() -> Self {
        ZmqServer
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
                
                println!("Credential string: {}", credential_string);
                match db_core.create_secret(payload, expiration, &credential) {
                    Ok(secret) => json!({"status": "success", "id": secret.id, "credential": credential_string}).to_string(),
                    Err(e) => json!({"status": "error", "message": e}).to_string(),
                }
            }
            Some("READ") => {
                let id = v["id"].as_str().unwrap_or("");
                let credential = v["credential"].as_str().unwrap_or("");

                let credential_from_string = Credential::new_from_string(&credential.to_string());

                match db_core.read_secret(id) {
                    Ok(Some(secret)) => json!({"status": "success", "payload": secret.decrypt(&credential_from_string)}).to_string(),
                    Ok(None) => json!({"status": "error", "message": "Secret not found or expired"}).to_string(),
                    Err(e) => json!({"status": "error", "message": e}).to_string(),
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
}

impl Server for ZmqServer {
    fn start(&self, db_core: &DatabaseCore) {
        let context = zmq::Context::new();
        let responder = context.socket(zmq::REP).unwrap();
        assert!(responder.bind("tcp://*:5555").is_ok());

        log!("INFO", "ZMQ Server running on port 5555");

        loop {
            let request = responder.recv_string(0).unwrap().unwrap();
            log!("INFO", &format!("{}", request));

            let response = ZmqServer::handle_request(db_core, &request);

            responder.send(&response, 0).unwrap();
        }
    }
}
