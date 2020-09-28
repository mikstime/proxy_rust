use hyper::{Response, Request, Client, Body};
use std::result::Result;
use crate::proxy::proxy_handler::tunnel;
type HttpClient = Client<hyper::client::HttpConnector>;

mod host_addr;
use host_addr::host_addr;

pub async fn https_request(client: HttpClient, acceptor: tokio_rustls::TlsAcceptor, req: Request<Body>) -> Result<Response<Body>, hyper::Error>  {
    let uri_str = format!("{}", req.uri().host().unwrap());
    if let Some(addr) = host_addr(req.uri()) {
        tokio::task::spawn(async move {
            match req.into_body().on_upgrade().await {
                Ok(upgraded) => {
                    if let Err(e) = tunnel::tunnel(upgraded, uri_str, acceptor, addr).await {
                        eprintln!("server io error: {}", e);
                    };
                }
                Err(e) => eprintln!("upgrade error: {}", e),
            }
        });

        Ok(Response::new(Body::empty()))
    } else {
        let mut resp = Response::new(
            Body::from("CONNECT must be to a socket address")
        );

        *resp.status_mut() = http::StatusCode::BAD_REQUEST;

        Ok(resp)
    }
}