use std::net::UdpSocket;
use std::env;
use simple_logger::SimpleLogger;

use crate::dnsserver::handle_query;

mod k8s;
mod dnsserver;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    SimpleLogger::new().env().init().unwrap();

    let ifaddr = env::var("POD_IP").unwrap_or("0.0.0.0".to_string());
    let dnsport = env::var("DNS_PORT").unwrap_or("53".to_string());

    log::info!("Starting kube-ingress-dns on {}:{}", ifaddr, dnsport);

    // Bind an UDP socket on port 53
    let socket = UdpSocket::bind((ifaddr, dnsport.parse::<u16>().unwrap()))?;

    // For now, queries are handled sequentially, so an infinite loop for servicing
    // requests is initiated.
    loop {
        match handle_query(&socket).await {
            Ok(_) => {}
            Err(e) => log::error!("{}", e),
        }
    }
}