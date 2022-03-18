use std::net::SocketAddr;

pub struct Config {
    pub forward_listen: Vec<SocketAddr>,
    pub enable_forward: bool,

    pub bootstrap_nodes: Vec<SocketAddr>,
}
