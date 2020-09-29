use hyper::{Response, Request, Client, Body};
use std::result::Result;
use crate::proxy::proxy_handler::tunnel;
type HttpClient = Client<hyper::client::HttpConnector>;

use futures::stream::{TryStreamExt};

use async_std::fs::File;
use async_std::io::prelude::*;
use chrono::Utc;

mod host_addr;
use host_addr::host_addr;

pub async fn store_request(req: Request<Body>) -> Request<Body> {
    let (parts, body) = req.into_parts();

    let first_line = format!("{} {} {:?}\r\n", parts.method, parts.uri, parts.version);
    let mut headers_lines = String::new();

    for (key, val) in &parts.headers {
        headers_lines += &format!("{}: {}\r\n", key.as_str(), String::from_utf8_lossy((*val).as_bytes()));
    }

    let now = Utc::now();

    let host = match parts.uri.host() {
        Some(h) => h,
        None => "localhost"
    };
    let file_name = format!("./requests/{}|||{}|||{}|||{}", uuid::Uuid::new_v4(), parts.method, host, now.timestamp_millis());
    let stored_req = format!("{}{}\r\n", first_line, headers_lines);

    let req = Request::from_parts(parts, body);

    let mut file = match File::create(file_name).await {
        Ok(f) => f,
        Err(e) => {
            println!("Не удалось сохранить запрос =c");
            println!("Информация об ошибке: {}", e);
            return req
        },
    };
    if let Err(e) = file.write_all(stored_req.as_bytes()).await {
        println!("Не удалось сохранить запрос =c");
        println!("Информация об ошибке: {}", e);
    }

    req
}
pub async fn https_request(_client: HttpClient, acceptor: tokio_rustls::TlsAcceptor, req: Request<Body>) -> Result<Response<Body>, hyper::Error>  {
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