use std::net::SocketAddr;

#[derive(Debug)]
pub struct REST {
    pub host: SocketAddr,
}

impl REST {
    pub fn new(host: &str, port: u16) -> Result<REST, String> {
        let host = format!("{}:{}", host, port);
        let host = match host.parse() {
            Ok(host) => host,
            Err(_) => return Err(format!("Invalid host: {}", host)),
        };
        Ok(REST{ host })
    }
}
