use super::AccountSummary;
use anyhow::Result;
use log;
use tokio::sync::oneshot::Receiver;
/// # Errors
///
/// Will return `Err` if `Receiver` will be unable to receive message
pub async fn handle(input: Receiver<Vec<AccountSummary>>) -> Result<()> {
    let mut data = input.await?;
    data.sort_unstable_by_key(|x| x.client);

    let mut serializer = csv_async::AsyncWriterBuilder::new()
        .flexible(true)
        .has_headers(true)
        .create_serializer(tokio::io::stdout());

    for record in &data {
        match serializer.serialize(&record).await {
            Ok(var) => {
                log::debug!("Printed {var:?}");
            }
            Err(err) => log::error!("Error with record {err}"),
        };
    }

    Ok(())
}
