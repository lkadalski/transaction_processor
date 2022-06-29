use anyhow::Result;
use clap::Parser;
use rand::prelude::*;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use transaction_processor::{ClientId, Transaction, TransactionId};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, long)]
    record_count: u32,
}
#[cfg(feature = "generate")]
#[tokio::main]
async fn main() -> Result<()> {
    use transaction_processor::TransactionType;

    let args = Arguments::parse();
    let mut rng = rand::thread_rng();
    let mut wtr = csv_async::AsyncWriterBuilder::new()
        .flexible(true)
        .create_serializer(tokio::io::stdout());
    let mut transactions = vec![];
    let mut last_client_id = 1_u16;
    let mut last_disputed = 1u32;
    let mut last_disputed_client_id = 1u16;
    let mut tx_id = 1u32;
    let mut clients = vec![last_client_id];
    for _record in 0..args.record_count {
        let generated_transaction = match rng.gen_range(0u16..100) {
            0..=25 => {
                let client = if rng.gen_bool(0.2) {
                    let mut client = last_client_id;
                    client += 1;
                    clients.push(client);
                    client
                } else {
                    clients[rng.gen_range(0..clients.len())]
                };

                let transaction_id = tx_id;
                tx_id += 1;
                last_client_id = client;
                transactions.push((transaction_id, client));

                Transaction {
                    tx_type: TransactionType::Deposit,
                    client_id: client,
                    transaction_id,
                    amount: generate_decimal(&mut rng),
                }
            }
            26..=50 => {
                let transaction_id = tx_id;
                tx_id += 1;

                let client = clients[rng.gen_range(0..clients.len())];

                Transaction {
                    tx_type: TransactionType::Withdrawal,
                    client_id: client,
                    transaction_id,
                    amount: generate_decimal(&mut rng),
                }
            }
            51..=70 => {
                if transactions.is_empty() {
                    continue;
                }
                let (transaction_id, client) = transactions[rng.gen_range(0..transactions.len())];
                last_disputed = transaction_id;
                last_disputed_client_id = client;
                Transaction {
                    tx_type: TransactionType::Dispute,
                    client_id: client,
                    transaction_id,
                    amount: None,
                }
            }
            71..=95 => {
                if transactions.is_empty() {
                    continue;
                }

                Transaction {
                    tx_type: TransactionType::Resolve,
                    client_id: last_disputed_client_id,
                    transaction_id: last_disputed,
                    amount: None,
                }
            }
            // Low probability because with enough transactions, most users were ending up in the locked state.
            // which makes sense.
            96..=100 => {
                if transactions.is_empty() {
                    continue;
                }
                Transaction {
                    tx_type: TransactionType::ChargeBack,
                    client_id: last_disputed_client_id,
                    transaction_id: last_disputed,
                    amount: None,
                }
            }
            _ => unreachable!(),
        };
        wtr.serialize(generated_transaction).await?;
        wtr.flush().await?;
    }
    Ok(())
}

fn generate_decimal(rng: &mut ThreadRng) -> Option<rust_decimal::Decimal> {
    Decimal::from_f64(rng.gen_range(0.0_f64..1000.0)).map(|num| num.round_dp(4))
}
