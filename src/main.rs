mod cli;
mod proxy;
#[async_std::main]
async fn main() -> std::io::Result<()> {
    match futures::try_join!(
        cli::run(),
        proxy::run(),
    ) {
        Err(e) => {
            println!("Ошибка в процессе выполнения программы.");
            println!("Информация об ошибке: {}", e);
        }
        _ => {}
    }
    Ok(())
}