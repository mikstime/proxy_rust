use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Server};

type HttpClient = Client<hyper::client::HttpConnector>;
mod proxy_handler;
use proxy_handler::proxy;

pub async fn run() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 1338));
    let client = HttpClient::new();

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}