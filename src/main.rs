mod cli;
mod proxy;
#[async_std::main]
async fn main() -> std::io::Result<()> {
    futures::join!(
        cli::run(),
        proxy::run(),
    );
    Ok(())
}