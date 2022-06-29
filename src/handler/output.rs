use anyhow::Result;
use tokio::sync::oneshot::Receiver;

use super::AccountSummary;

pub async fn handle(input: Receiver<Vec<AccountSummary>>) -> Result<()> {
    let data = input.await.unwrap();
    let mut serializer = csv_async::AsyncWriterBuilder::new()
        .flexible(true)
        .has_headers(true)
        .create_serializer(tokio::io::stdout());
    for record in data {
        match serializer.serialize(&record).await {
            Ok(_var) => {
                // println!("Serialized {var:?}");
            }
            Err(err) => eprintln!("Errorek {err}"),
        };
    }

    Ok(())
}
