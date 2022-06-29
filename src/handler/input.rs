use crate::Transaction;
use anyhow::Result;
use futures::stream::StreamExt;
use tokio::sync::mpsc::{self, Sender};
/// .
/// Handle files or stdin
///
/// # Errors
///
/// This function will return an error if File not exists or could not deserialize record.
pub async fn handle(input: Option<String>) -> Result<mpsc::Receiver<Transaction>> {
    let (tx, rx) = mpsc::channel(1000);
    tokio::spawn(async move {
        if let Err(err) = process(input, tx).await {
            eprintln!("Could not process input due {err}");
            std::process::exit(1);
        }
    });
    Ok(rx)
}

async fn process(input: Option<String>, tx: Sender<Transaction>) -> Result<()> {
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
        tx.send(record).await?;
    }
    Ok(())
}
