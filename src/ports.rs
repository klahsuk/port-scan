use std::{net::{SocketAddr, ToSocketAddrs}, time::Duration};
use crate::{common_ports::MOST_COMMON_PORTS_100, model::{Port, Subdomain}};
use tokio::{spawn, sync::mpsc};
use::tokio::net::TcpStream;
use futures::StreamExt;

/// iterates over the most common ports 
/// scans each port on the domain
/// checks if port is open
/// changes the open_ports in subdomain 
/// returns the subdomain

pub async fn scan_ports(concurrency: usize, mut subdomain: Subdomain) -> Subdomain{
    //need vec as string -> to _socket_addrs gives something that is an iterator of type SocketAddr
    let mut ret = subdomain.clone();
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.is_empty() {
        return subdomain;
    }

    let socket_address = socket_addresses[0];
    // Concurrent stream method 3: using channels
    let (input_tx, input_rx) = mpsc::channel(concurrency);
    let (output_tx, output_rx) = mpsc::channel(concurrency);

    tokio::spawn(async move {
        for port in MOST_COMMON_PORTS_100 {
            let _ = input_tx.send(*port).await;
        }
    });

    let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
    input_rx_stream
        .for_each_concurrent(concurrency, |port|{
            let output_tx = output_tx.clone();
            async move {
                let port = scan_port(socket_address, port).await;
                if port.is_open {
                    let _ = output_tx.send(port).await;
                }
            }
        }).await;

        drop(output_tx);
        let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);

        ret.open_ports = output_rx_stream.collect().await;

        ret

}


/// create timeout value
/// set port on the socket addrs using the mut ref
/// try connecting and decide if port is open or not
/// returns the Port struct with its is_open status

async fn scan_port(mut socket_addrs: SocketAddr, port: u16) -> Port{
    let timeout = Duration::from_secs(3);
    socket_addrs.set_port(port);

    let is_open = matches!(
        tokio::time::timeout(timeout, TcpStream::connect(&socket_addrs)).await,
        Ok(Ok(_))
    );
    Port { port, is_open }
}