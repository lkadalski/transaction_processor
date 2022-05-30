use std::{pin::Pin, sync::Arc};

use anyhow::Result;
use futures::{stream::StreamExt, AsyncRead};
use tokio::io::AsyncBufRead;

use crate::Transaction;
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
    let (tx, rx) = tokio::sync::mpsc::channel(1000);
    //decide what input
    // let reader: Arc<dyn AsyncRead> = match input {
    //     Some(filename) => Arc::new(<dyn tokio::io::AsyncRead>::new(
    //         tokio::fs::File::open(filename).await?,
    //     )),
    //     None => Arc::new(<dyn tokio::io::AsyncRead>::new(tokio::io::stdin())),
    // };
    let reader: Box<dyn tokio::io::AsyncBufRead + Send + Unpin> = if let Some(file) = input {
        Box::new(tokio::io::BufReader::new(
            tokio::fs::File::open(file).await?,
        ))
    } else {
        Box::new(tokio::io::BufReader::new(tokio::io::stdin()))
    };
    let mut csv_reader = csv_async::AsyncReaderBuilder::new()
        .flexible(true)
        .create_deserializer(reader);
    let mut records = csv_reader.deserialize();
    while let Some(record) = records.next().await {
        let record: Transaction = record?;
        println!("{record:?}");
        tx.send(record).await?;
    }
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
