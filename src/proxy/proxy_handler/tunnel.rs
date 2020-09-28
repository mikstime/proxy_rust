use std::net::{SocketAddr, ToSocketAddrs};
use futures_util::future::try_join;
use hyper::upgrade::Upgraded;
use hyper::{Body, Client, Method, Request, Response};
use rustls::internal::pemfile;
use tokio::net::TcpStream;

use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{Certificate, NoClientAuth, ServerConfig};
use futures_util::TryStreamExt;
use std::io;
use std::result::Result;
// Create a TCP connection to host:port, build a tunnel between the connection and
// the upgraded connection
pub async fn tunnel(mut upgraded: Upgraded, uri: String, acceptor: tokio_rustls::TlsAcceptor, addr: SocketAddr) -> std::io::Result<()> {

    let mut upgraded = acceptor.accept(upgraded).await?;

    use tokio_rustls::{rustls::ClientConfig, webpki::DNSNameRef, TlsConnector};
    let mut config = ClientConfig::new();

    let pem = std::process::Command::new("./certs/gen_cert.sh")
        .args(&[uri.clone(), "5436723487".to_string()])
        .output()
        .expect("failed to execute process");
    let mut pem = std::io::BufReader::new(&pem.stdout[..]);
//////////////////////////////DEFAULT CERTS////////////////////////////////////
//    config
//        .root_store
//        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
///////////////////////////////////////////////////////////////////////////////
    config
        .root_store
        .add_pem_file(&mut pem)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))?;

    let connector = TlsConnector::from(std::sync::Arc::new(config));
    let stream = tokio::net::TcpStream::connect(addr).await?;
    let domain = DNSNameRef::try_from_ascii_str(&uri)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?;
    let mut stream = connector.connect(domain, stream).await?;

    // Proxying data
    {
        let (mut server_rd, mut server_wr) = tokio::io::split(stream);
        let (mut client_rd, mut client_wr) = tokio::io::split(upgraded);

        let client_to_server = tokio::io::copy(&mut client_rd, &mut server_wr);
        let server_to_client = tokio::io::copy(&mut server_rd, &mut client_wr);
        try_join(client_to_server, server_to_client).await?;
    }

    Ok(())
}