use async_std::prelude::*;
use hyper::body::HttpBody;
use tokio::io::{stdout, AsyncWriteExt};

use crate::cli::Config;
use crate::proxy::proxy_handler::handle::http_request::store_request;
//request f10b6b4e-9ab0-4f73-a7c7-0d3fd1bfaa7e send
pub async fn send_request(req: &str) {
    let req = parse_request(req.to_string()).await;
    let req = store_request(req).await;
    let client = hyper::Client::new();
    let mut resp = client.request(req).await.unwrap();

    while let Some(chunk) = resp.body_mut().data().await {
        use async_std::io::stdout;
        stdout().write_all(&chunk.unwrap()).await;
    }
}
pub async fn parse_request(req: String) -> hyper::Request<hyper::Body> {
    let req = req.clone();
    let split_request = req.split("\r\n\r\n").collect::<Vec<&str>>();
    let before_body = split_request[0].split("\r\n").collect::<Vec<&str>>();
    let first_line_split = before_body[0].split(" ").collect::<Vec<&str>>();

    let method = first_line_split[0];
    let uri = first_line_split[1];
    let version = first_line_split[2];
    let version = match version {
        "HTTP/0.9" => http::version::Version::HTTP_09,
        "HTTP/1.0" => http::version::Version::HTTP_10,
        "HTTP/2.0" => http::version::Version::HTTP_2,
        "HTTP/3.0" => http::version::Version::HTTP_3,
        _ => http::version::Version::HTTP_11,
    };
    let body_string = String::from(split_request[1]);
    let body = hyper::Body::from(body_string);

    let mut builder = hyper::Request::builder()
        .method(http::method::Method::from_bytes(method.as_bytes()).unwrap())
        .uri(uri.parse::<http::uri::Uri>().unwrap())
        .version(version);

    let mut skip = 0;

    for header in before_body.iter() {
        if skip == 0 {
            skip = 1;
            continue
        }
        let split_header = header.split(":").collect::<Vec<&str>>();
        builder = builder.header(
            http::header::HeaderName::from_bytes(split_header[0].as_bytes()).unwrap(),
            split_header[1].parse::<http::header::HeaderValue>().unwrap()
        );
    }
    builder.body(body).unwrap()
}
pub async fn scan_request(req: &str) {
    let req = parse_request(req.to_string());
}
pub async fn print_request(req: &str) {
    println!("{}",req);
}

pub async fn load_request(id: &str) -> std::io::Result<String> {

    let mut entries = async_std::fs::read_dir("./requests").await?;

    while let Some(res) = entries.next().await {
        let entry = res?;

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.starts_with(id) {
            return async_std::fs::read_to_string(&format!("./requests/{}", file_name)).await;
        }
    }
    use std::io::{Error, ErrorKind};
    Err(Error::new(ErrorKind::Other, "Файл не найден"))
}
pub async fn show_request(line: &str, config: &mut Config) -> std::io::Result<()> {
    let split_line = line.split(" ").collect::<Vec<&str>>();
    if split_line.len() >= 2 {
        let id = split_line[1];
        let req = match load_request(id).await {
            Ok(v) => v,
            Err(e) => {
                println!("Не удалось найти запрос.");
                return Err(e)
            }
        };
        if split_line.len() == 3 && split_line[2] == "scan" {
            scan_request(&req).await;
        } else if split_line.len() == 3 && split_line[2] == "send" {
            send_request(&req).await;
        } else {
            print_request(&req).await;
        }
    } else {
        println!(
r"===============================================================================
request <id> – показать подробную информацию о запросе
    scan – проверить запрос на уязвимость
    send - повторно отправить запрос
===============================================================================");
    }
    Ok(())
}