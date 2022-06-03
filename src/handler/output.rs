use anyhow::Result;
use tokio::sync::oneshot::Receiver;

use crate::DataSource;

pub async fn handle(input: Receiver<DataSource>) -> Result<()> {
    let data = input.await;
    let mut serializer =
        csv_async::AsyncWriterBuilder::new().create_serializer(tokio::io::stdout());

    for entry in data.unwrap().iter() {
        serializer.serialize(entry).await?;
    }
    Ok(())
}
