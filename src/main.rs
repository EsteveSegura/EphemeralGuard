use secret_db::server::factory::{ServerFactory, ServerType};

#[macro_use]
mod utils;

fn main() {
   log!("INFO", "EphemeralGuard running...");

   let server = ServerFactory::create_server(ServerType::ZMQ);
   server.start();
}