use ephemeral_guard::server::factory::{ServerFactory, ServerType};
use ephemeral_guard::db::core::DatabaseCore;
use ephemeral_guard::config;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::process::exit;

#[macro_use]
mod utils;

fn main() {
    log!("INFO", "EphemeralGuard running...");

    let db_file_path = "database.bin";
    let db_core = match DatabaseCore::from_file(db_file_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to load database: {}", e);
            exit(1);
        }
    };

    let db_core_for_cleanup = db_core.clone();
    let db_core_for_server = db_core.clone();

    let (tx, rx) = mpsc::channel();
    Arc::new(Mutex::new(Some(thread::spawn(move || {
        let cleanup_interval = Duration::from_secs(config::CLEANUP_INTERVAL_SECONDS);
        let mut last_cleanup = Instant::now();

        loop {
            if let Ok(_) | Err(mpsc::TryRecvError::Disconnected) = rx.try_recv() {
                break;
            }

            if last_cleanup.elapsed() >= cleanup_interval {
                log!("INFO", "Running cleanup thread...");
                db_core_for_cleanup.clean_expired_secrets(400, 1);
                last_cleanup = Instant::now();
            }

            thread::sleep(Duration::from_millis(100));
        }

        log!("INFO", "Cleanup thread terminating...");
        db_core_for_cleanup.save_to_file(db_file_path).expect("Failed to save database.");
    }))));

    Arc::new(Mutex::new(Some(thread::spawn(move || {
        match config::DEFAULT_SERVER {
            config::DefaultTypeServer::ZMQ => {
                let server_zmq = ServerFactory::create_server(ServerType::ZMQ);
                server_zmq.start(&db_core_for_server);
            }
            config::DefaultTypeServer::TCP => {
                let server_tcp = ServerFactory::create_server(ServerType::TCP);
                server_tcp.start(&db_core_for_server);
            }
        }

        log!("INFO", "Server thread terminating...");
    }))));

    ctrlc::set_handler(move || {
        log!("INFO", "Shutting down EphemeralGuard...");
        let _ = tx.send(());

        db_core.save_to_file(db_file_path).expect("Failed to save database.");
        exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
