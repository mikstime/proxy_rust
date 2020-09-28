use hyper::{Response, Request, Client, Body};
use std::result::Result;
type HttpClient = Client<hyper::client::HttpConnector>;

use futures::stream::{TryStreamExt};

use async_std::fs::File;
use async_std::io::prelude::*;
use chrono::Utc;

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
    let now = Utc::now();

    let file_name = format!("./requests/{}|||{}|||{}|||{}", uuid::Uuid::new_v4(), parts.method, parts.uri.host().unwrap(), now.timestamp_millis());
    let stored_req = format!("{}{}\r\n{}", first_line, headers_lines, body_string);

    let body = Body::from(body_string);
    let req = Request::from_parts(parts, body);

    let mut file = match File::create(file_name).await {
        Ok(f) => f,
        Err(err) => return req,
    };
    file.write_all(stored_req.as_bytes()).await;

    req
}
pub async fn http_request(client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let req = store_request(req).await;
    client.request(req).await
}

