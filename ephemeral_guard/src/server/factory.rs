use crate::server::traits::Server;
use crate::server::tcp_server::TcpServer;

pub enum ServerType {
    TCP
}

pub struct ServerFactory;

impl ServerFactory {
    pub fn create_server(server_type: ServerType) -> Box<dyn Server> {
        match server_type {
            ServerType::TCP => Box::new(TcpServer::new()),
        }
    }
}
