use anyhow::Result;
use std::env;
use transaction_processor::handler;

#[tokio::main]
async fn main() -> Result<()> {
    let input = env::args().nth(1);

    env_logger::init();

    if let Err(err) = handler::output::handle(
        handler::transaction::handle(handler::input::handle(input).await?).await?,
    )
    .await
    {
        log::error!("Error occured during processing: {err}");
        std::process::exit(1)
    }
    Ok(())
}
