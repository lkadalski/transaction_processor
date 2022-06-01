use anyhow::Result;
use tokio::sync::oneshot::Receiver;

use crate::DataSource;

pub async fn handle(input: Receiver<DataSource>) -> Result<()> {
    let data = input.await;
    for entry in &data {
        println!("{entry:?}");
    }
    Ok(())
}
