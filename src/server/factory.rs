use crate::server::traits::Server;
use crate::server::zmq_server::ZmqServer;

pub enum ServerType {
    ZMQ,
}

pub struct ServerFactory;

impl ServerFactory {
    pub fn create_server(server_type: ServerType) -> Box<dyn Server> {
        match server_type {
            ServerType::ZMQ => Box::new(ZmqServer::new()),
        }
    }
}
