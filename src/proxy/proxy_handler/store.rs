use hyper::Request;

fn serialize(parts: Parts) -> serde_json::Result<Parts>
{
    Ok(parts)
}
const SEP: &str = "SDFBAJDGFKABDKJFGANWEJFOJ23R32NRID9232OHF340RJ230D23HF230HF349HF2349RF3J49HF3";
type Parts = (http::method::Method, http::uri::Uri, http::version::Version, http::header::HeaderMap<http::header::HeaderValue>);
pub async fn store_request(parts: Parts) -> std::io::Result<()> {
//    let scheme = parts.1.scheme().unwrap();
//    let authority = parts.1.authority().unwrap();
//    let path_and_query = parts.1.path_and_query().unwrap();
//    let str_to_store = format!( "{}{sep}{}{sep}{}{sep}{}{sep}{:?}{sep}{:?}", parts.0, scheme, authority, path_and_query,parts.2,parts.3, sep = SEP);
//    load_request(str_to_store).await?;
    Ok(())
}
pub async fn load_request(raw: String) -> std::io::Result<()> {

    let split: Vec<String> = raw.split(SEP).map(String::from).collect();
    let method_raw = split[0].as_bytes();
    let scheme_raw = split[1].clone().parse::<http::uri::Scheme>().unwrap();
    let authority_raw = split[2].clone().parse::<http::uri::Authority>().unwrap();
    let path_and_query_raw = split[3].clone().parse::<http::uri::PathAndQuery>().unwrap();
    let version_raw = split[4].clone();
    let headers_str = split[5].clone();

    let version = match version_raw.as_ref() {
        "HTTP/0.9" => http::version::Version::HTTP_09,
        "HTTP/1.0" => http::version::Version::HTTP_10,
        "HTTP/2.1" => http::version::Version::HTTP_2,
        "HTTP/3.0" => http::version::Version::HTTP_3,
        _ => http::version::Version::HTTP_11,
    };

    let uri = hyper::Uri::builder()
        .scheme(scheme_raw)
        .path_and_query(path_and_query_raw)
        .authority(authority_raw)
        .build().unwrap();

    let method = hyper::Method::from_bytes(method_raw).unwrap();
    println!("{}", headers_str);
    let req = Request::builder().version(version).uri(uri).method(method).body("");
    Ok(())
}
//fn serialize<T>(req: Request<T>) -> serde_json::Result<Request<Vec<u8>>>
//    where T: ser::Serialize,
//{
//    let (parts, body) = req.into_parts();
//    let body = serde_json::to_vec(&body)?;
//    Ok(Request::from_parts(parts, body))
//}