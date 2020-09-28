use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Server};

type HttpClient = Client<hyper::client::HttpConnector>;
pub mod proxy_handler;
use proxy_handler::proxy;

fn error(err: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

pub async fn run() -> std::io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 1337));
    let client = HttpClient::new();



//    // Build TLS configuration.
//    let tls_cfg = {
//        // Load public certificate.
//        let certs = load_certs("certs/ca.crt")?;
//        // Load private key.
//        let key = load_private_key("certs/ca.key")?;
//        // Do not use client certificate authentication.
//        let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
//        // Select a certificate to use.
//        cfg.set_single_cert(certs, key)
//            .map_err(|e| error(format!("{}", e)))?;
//        // Configure ALPN to accept HTTP/2, HTTP/1.1 in that order.
//        cfg.set_protocols(&[b"h2".to_vec(), b"http/1.1".to_vec()]);
//        std::sync::Arc::new(cfg)
//    };
//
//    use tokio_rustls::TlsAcceptor;
//    let tls_acceptor = TlsAcceptor::from(tls_cfg);
    use tokio_rustls::rustls::internal::pemfile::{certs, rsa_private_keys};
    use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
    use tokio_rustls::TlsAcceptor;
    use std::fs::File;
    use std::sync::Arc;
    use std::io::{self, BufReader};
    use std::path::{Path};

    fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
        certs(&mut BufReader::new(File::open(path)?))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
    }

    fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
        rsa_private_keys(&mut BufReader::new(File::open(path)?))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
    }

    let cert_path = Path::new("./certs/ca.crt");
    let key_path = Path::new("./certs/ca.key");
    let certs = load_certs(&cert_path)?;
    let mut keys = load_keys(&key_path)?;
    let mut config = ServerConfig::new(NoClientAuth::new());
    config
        .set_single_cert(certs, keys.remove(0))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    let acceptor = TlsAcceptor::from(Arc::new(config));






    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        let acceptor = acceptor.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), acceptor.clone(), req))) }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}