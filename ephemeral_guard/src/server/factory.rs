use crate::server::traits::Server;
use crate::server::zmq_server::ZmqServer;
use crate::server::tcp_server::TcpServer;

pub enum ServerType {
    ZMQ,
    TCP
}

pub struct ServerFactory;

impl ServerFactory {
    pub fn create_server(server_type: ServerType) -> Box<dyn Server> {
        match server_type {
            ServerType::ZMQ => Box::new(ZmqServer::new()),
            ServerType::TCP => Box::new(TcpServer::new()),
        }
    }
}
