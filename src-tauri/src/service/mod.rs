pub mod client;
pub mod server;

use std::{collections::HashSet, net::IpAddr};

use mdns_sd::TxtProperties;

#[derive(Debug, Clone)]
pub struct Server {
    host: String,
    addresses: HashSet<IpAddr>,
    port: u16,
    properties: TxtProperties,
}

impl Server {
    pub fn new(
        host: String,
        address: HashSet<IpAddr>,
        port: u16,
        properties: TxtProperties,
    ) -> Self {
        Self { host, addresses: address, port, properties }
    }

    pub fn addresses(&self) -> &HashSet<IpAddr> {
        &self.addresses
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn properties(&self) -> &TxtProperties {
        &self.properties
    }

    pub fn host(&self) -> &str {
        &self.host
    }
}
