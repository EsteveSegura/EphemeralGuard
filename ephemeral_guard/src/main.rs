use ephemeral_guard::server::factory::{ServerFactory, ServerType};
use ephemeral_guard::db::core::DatabaseCore;
use ephemeral_guard::config;
use std::thread;
use std::time::{Duration, Instant};

#[macro_use]
mod utils;

fn main() {
   log!("INFO", "EphemeralGuard running...");

   let db_core = DatabaseCore::new();
   let db_core_for_cleanup = db_core.clone();

   let batch_size = 400;
   let mut current_batch = 1;

   thread::spawn(move || {
      let cleanup_interval = Duration::from_secs(config::CLEANUP_INTERVAL_SECONDS);
      let mut last_cleanup = Instant::now();

      loop {
          if last_cleanup.elapsed() >= cleanup_interval {
              log!("INFO", "Running cleanup thread...");
              db_core_for_cleanup.clean_expired_secrets(batch_size, current_batch);

              let total_expirations = db_core_for_cleanup.get_total_expirations();
              let total_batches = (total_expirations + batch_size - 1) / batch_size;

              current_batch += 1;
              if current_batch > total_batches || total_batches == 0 {
                  current_batch = 1;
              }

              last_cleanup = Instant::now();
          }

          thread::sleep(Duration::from_millis(500));
      }
  });

    match config::DEFAULT_SERVER {
        config::DefaultTypeServer::ZMQ => {
            let server_zmq = ServerFactory::create_server(ServerType::ZMQ);
            server_zmq.start(&db_core);
        }
        config::DefaultTypeServer::TCP => {
            let server_tcp = ServerFactory::create_server(ServerType::TCP);
            server_tcp.start(&db_core);
        }
    }  
}