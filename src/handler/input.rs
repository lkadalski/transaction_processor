use anyhow::Result;

use tokio::futures::stream::StreamExt;

use tokio::io::AsyncRead;

use crate::Transaction;

use super::{output, transaction};

pub fn handle(input: Option<String>) -> Result<()> {
    //tokio start
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(initialize(input));
    //tokio end
    Ok(())
}

async fn initialize(input: Option<String>) {
    if let Err(err) = process(input).await {
        eprintln!("Could not process input due {err}");
        std::process::exit(1);
    }
}
//Handle files or stdin
//CSV READER
async fn process(input: Option<String>) -> Result<()> {
    //potentially could be stdin, TODO
    let mut csv_reader = csv_async::AsyncReaderBuilder::new()
        .flexible(true)
        .create_deserializer(tokio::fs::File::open(input.unwrap()).await?);
    let records = csv_reader.deserialize::<Transaction>();
    records.next()?;
    //     output::OutputHandler::handle(transaction::TransactionHandler::handle(reader).await?).await;
    Ok(())
}
// // async fn process(input: Option<String>) -> Result<()> {
//     let reader: Box<dyn tokio::io::AsyncBufRead> = match input {
//         None => Box::new(tokio::io::BufReader::new(tokio::io::stdin())),
//         Some(filename) => Box::new(tokio::io::BufReader::new(
//             tokio::fs::File::open(filename).await?,
//         )),
//     };
//     output::OutputHandler::handle(transaction::TransactionHandler::handle(reader).await?).await;
//     Ok(())
