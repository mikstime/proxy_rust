use hyper::{Response, Request, Client, Body};
use std::result::Result;
use crate::proxy::proxy_handler::tunnel;
type HttpClient = Client<hyper::client::HttpConnector>;
use std::net::{SocketAddr, ToSocketAddrs};

use futures::stream::{self, StreamExt, TryStreamExt};

pub async fn store_request(req: Request<Body>) -> Request<Body> {
    let (parts, body) = req.into_parts();

    let first_line = format!("{} {} {:?}\r\n", parts.method, parts.uri, parts.version);
    let mut headers_lines = String::new();
    for (key, val) in &parts.headers {

        headers_lines += &format!("{}: {}\r\n", key.as_str(), String::from_utf8_lossy((*val).as_bytes()));
    }
    let entire_body = body
        .try_fold(Vec::new(), |mut data, chunk| async move {
            data.extend_from_slice(&chunk);
            Ok(data)
        })
        .await.unwrap();
    let body_string = String::from_utf8(entire_body).unwrap();
    let stored_req = format!("{}{}\r\n{}", first_line, headers_lines, body_string);
    let file_name = format!("./requests/{}", parts.method, parts.uri, );
    async_std::fs::File::create(file_name).await?;
    println!("{}", stored_req);
    let body = Body::from(body_string);
    let req = Request::from_parts(parts, body);
    req
}
pub async fn http_request(client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let req = store_request(req).await;
    client.request(req).await
}

