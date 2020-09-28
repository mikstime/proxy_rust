mod cli;
mod proxy;
#[async_std::main]
async fn main() -> std::io::Result<()> {
    futures::join!(
//        cli::run(),
        proxy::run(),
    );
    Ok(())
}
//
//
//use std::fs::File;
//use std::io::{self, BufReader};
//use std::net::ToSocketAddrs;
//use std::path::{Path, PathBuf};
//use std::sync::Arc;
//use tokio::io::{copy, sink, split, AsyncWriteExt};
//use tokio::net::TcpListener;
//use tokio_rustls::rustls::internal::pemfile::{certs, rsa_private_keys};
//use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
//use tokio_rustls::TlsAcceptor;
//
//
//fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
//    certs(&mut BufReader::new(File::open(path)?))
//        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
//}
//
//fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
//    rsa_private_keys(&mut BufReader::new(File::open(path)?))
//        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
//}
//
//#[tokio::main]
//async fn main() -> io::Result<()> {
//    use std::net::SocketAddr;
//    let addr = SocketAddr::from(([127, 0, 0, 1], 1338));
//
//    let cert_path = Path::new("./certs/ca.crt");
//    let key_path = Path::new("./certs/ca.key");
//    let certs = load_certs(&cert_path)?;
//    let mut keys = load_keys(&key_path)?;
//    let flag_echo = false;
//
//    let mut config = ServerConfig::new(NoClientAuth::new());
//    config
//        .set_single_cert(certs, keys.remove(0))
//        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
//    let acceptor = TlsAcceptor::from(Arc::new(config));
//
//    let mut listener = TcpListener::bind(&addr).await?;
//
//    loop {
//        let (stream, peer_addr) = listener.accept().await?;
//        let acceptor = acceptor.clone();
//
//        let fut = async move {
//            let mut stream = acceptor.accept(stream).await?;
//
//            if flag_echo {
//                let (mut reader, mut writer) = split(stream);
//                let n = copy(&mut reader, &mut writer).await?;
//                writer.flush().await?;
//                println!("Echo: {} - {}", peer_addr, n);
//            } else {
//                let mut output = sink();
//                stream
//                    .write_all(
//                        &b"HTTP/1.0 200 ok\r\n\
//                    Connection: close\r\n\
//                    Content-length: 12\r\n\
//                    \r\n\
//                    Hello world!"[..],
//                    )
//                    .await?;
//                stream.shutdown().await?;
//                copy(&mut stream, &mut output).await?;
//                println!("Hello: {}", peer_addr);
//            }
//
//            Ok(()) as io::Result<()>
//        };
//
//        tokio::spawn(async move {
//            if let Err(err) = fut.await {
//                eprintln!("{:?}", err);
//            }
//        });
//    }
//}