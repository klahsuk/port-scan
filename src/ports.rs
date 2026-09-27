use std::{net::{SocketAddr, TcpStream, ToSocketAddrs}, time::Duration};
use rayon::prelude::*;
use crate::{common_ports::MOST_COMMON_PORTS_100, model::{Port, Subdomain}};
use tokio::spawn;

/// iterates over the most common ports 
/// scans each port on the domain
/// checks if port is open
/// changes the open_ports in subdomain 
/// returns the subdomain

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain{
    //need vec as string -> to _socket_addrs gives something that is an iterator of type SocketAddr
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.is_empty() {
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port( socket_addresses[0], *port))
        .filter(|port| port.is_open)
        .collect();
    subdomain
}


/// create timeout value
/// set port on the socket addrs using the mut ref
/// try connecting and decide if port is open or not
/// returns the Port struct with its is_open status

pub fn scan_port(mut socket_addrs: SocketAddr, port: u16) -> Port{
    let timeout = Duration::from_secs(3);
    socket_addrs.set_port(port);

    let is_open = TcpStream::connect_timeout(&socket_addrs, timeout).is_ok();
    Port { port, is_open }
}