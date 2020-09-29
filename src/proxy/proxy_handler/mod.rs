use hyper::{Body, Client, Method, Request, Response};
use std::result::Result;

type HttpClient = Client<hyper::client::HttpConnector>;

mod tunnel;
pub mod handle;

pub async fn proxy(client: HttpClient, acceptor: tokio_rustls::TlsAcceptor,
                   req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    if Method::CONNECT == req.method() {
        handle::https_request::https_request(client, acceptor, req).await
    } else {
        handle::http_request::http_request(client, req).await
    }
}