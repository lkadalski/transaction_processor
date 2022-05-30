// use anyhow::Result;
// use tokio::{
//     fs::File,
//     io::{AsyncBufRead, AsyncBufReadExt},
//     sync::mpsc::Receiver,
// };

// use crate::Transaction;
// pub(crate) struct TransactionHandler {}
// impl TransactionHandler {
//     pub(crate) async fn handle(
//         reader: csv_async::AsyncDeserializer<File>,
//     ) -> Result<Receiver<Transaction>> {
//         let (tx, rx) = tokio::sync::mpsc::channel(1000);
//         tokio::spawn(async move {
//             let records = reader.deserialize::<Transaction>();
//         });
//         Ok(rx)
//     }
// }
