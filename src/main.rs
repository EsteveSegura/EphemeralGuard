use secret_db::server::factory::{ServerFactory, ServerType};
use secret_db::db::core::DatabaseCore;

#[macro_use]
mod utils;

fn main() {
   log!("INFO", "EphemeralGuard running...");

   let db_core = DatabaseCore::new();

   let server = ServerFactory::create_server(ServerType::ZMQ);
   server.start(&db_core);
}