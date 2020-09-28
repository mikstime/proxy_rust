mod show_help;
use show_help::show_help;
mod show_requests;
use show_requests::show_requests;
pub async fn run() -> std::io::Result<()> {
    use async_std::io;

    let stdin = io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).await? != 0 {
        let match_line = line.trim();
        match match_line.as_ref()  {
            "help" => show_help().await?,
            "request" => show_request(match_line).await?,
            _ => {},
        }
        line = "".to_string();
    }
    Ok(())
}

async fn show_request(_line: &str) -> std::io::Result<()> {
    show_requests(0i32).await;
    Ok(())
}