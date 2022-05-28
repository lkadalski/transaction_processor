use anyhow::Result;
use calculator::{ClientId, Transaction, TransactionId};
use clap::Parser;
use rand::prelude::*;
use rust_decimal::{prelude::FromPrimitive, Decimal};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, long)]
    record_count: u32,
}
#[cfg(feature = "generate")]
fn main() -> Result<()> {
    let args = Arguments::parse();
    let mut rng = rand::thread_rng();
    let mut wtr = csv::WriterBuilder::new()
        .flexible(true)
        .from_writer(std::io::stdout());
    let mut transactions = vec![];
    let mut client_id = 2_u16;
    let mut tx_id = 1u32;
    let mut clients = vec![ClientId(1_u16)];
    for _record in 0..args.record_count {
        let generated_transaction = match rng.gen_range(0..100) {
            0..=25 => {
                let client = if rng.gen_bool(0.2) {
                    let client = ClientId(client_id);
                    clients.push(client);
                    client_id += 1;
                    client
                } else {
                    clients[rng.gen_range(0..clients.len())]
                };

                let tx = TransactionId(tx_id);
                tx_id += 1;
                transactions.push((tx, client));

                Transaction::Deposit {
                    client,
                    tx,
                    amount: generate_decimal(&mut rng),
                }
            }
            26..=50 => {
                let tx = TransactionId(tx_id);
                tx_id += 1;

                let client = clients[rng.gen_range(0..clients.len())];

                Transaction::Withdrawal {
                    client,
                    tx,
                    amount: generate_decimal(&mut rng),
                }
            }
            51..=70 => {
                if transactions.is_empty() {
                    continue;
                }
                let (tx, client) = transactions[rng.gen_range(0..transactions.len())];

                Transaction::Dispute { client, tx }
            }
            71..=98 => {
                if transactions.is_empty() {
                    continue;
                }
                let (tx, client) = transactions[rng.gen_range(0..transactions.len())];

                Transaction::Resolve { client, tx }
            }
            // Low probability because with enough transactions, most users were ending up in the locked state.
            // which makes sense.
            99..=100 => {
                if transactions.is_empty() {
                    continue;
                }
                let (tx, client) = transactions[rng.gen_range(0..transactions.len())];

                Transaction::ChargeBack { client, tx }
            }
            _ => unreachable!(),
        };
        wtr.serialize(generated_transaction)?;
        wtr.flush()?;
    }
    Ok(())
}

fn generate_decimal(rng: &mut ThreadRng) -> rust_decimal::Decimal {
    Decimal::from_f64(rng.gen_range(0.0_f64..1000.0))
        .unwrap()
        .round_dp(4)
}
