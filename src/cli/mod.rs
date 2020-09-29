mod show_help;
use show_help::show_help;
mod show_requests;
use show_requests::show_requests;
mod show_request;
use show_request::show_request;
#[derive(Clone, Copy)]
pub struct Config {
    history: History,
}
impl Config {
    fn new() -> Config {Config{history: History::new()}}
}
#[derive(Clone, Copy)]
pub struct History {
    start: i32,
    end: i32,
}
impl History {
    fn new() -> History {History{start: 0, end: 10}}
}
pub async fn run() -> std::io::Result<()> {
    use async_std::io;
    let mut cfg = Config::new();
    let stdin = io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).await? != 0 {
        let match_line = line.trim();
        let mut first_word = String::new();

        for c in match_line.chars() {
            if c != ' ' {
                first_word += &c.to_string();
            } else {
                break;
            }
        }

        if let Err(e) = match first_word.as_ref() {
            "help" => show_help().await,
            "request" => show_request(match_line, &mut cfg).await,
            "history" => show_requests(match_line, &mut cfg).await,
            _ => {Ok(())},
        } {
            println!("Ошибка при выполнении команды: {}", line);
            println!("Информация об ошибке: {}", e);
        }
        line = "".to_string();
    }
    Ok(())
}