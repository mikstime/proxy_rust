use std::net::{SocketAddr, ToSocketAddrs};
pub fn host_addr(uri: &http::Uri) -> Option<SocketAddr> {
    uri.authority().and_then(|auth| {
        auth.as_str()
            .to_socket_addrs()
            .expect("Unable to parse connect header")
            .next()
    })
}