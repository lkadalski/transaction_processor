use crate::{Account, DataSource};
use anyhow::Result;

use tokio::sync::{mpsc::Receiver, oneshot};

use super::AccountSummary;

pub async fn handle(
    input: Receiver<crate::Transaction>,
) -> Result<oneshot::Receiver<Vec<AccountSummary>>> {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        if let Err(err) = process(input, tx).await {
            eprintln!("Could not process transaction due {err}");
            std::process::exit(1)
        }
    });
    Ok(rx)
}

async fn process(
    mut queue: Receiver<crate::Transaction>,
    tx: oneshot::Sender<Vec<AccountSummary>>,
) -> Result<()> {
    let mut data = DataSource::new();
    while let Some(message) = queue.recv().await {
        match message.tx_type {
            crate::TransactionType::Deposit => deposit(&mut data, message),
            crate::TransactionType::Withdrawal => withdrawal(&mut data, message),
            crate::TransactionType::Dispute => dispute(&mut data, message),
            crate::TransactionType::Resolve => resolve(&mut data, message),
            crate::TransactionType::ChargeBack => charge_back(&mut data, message),
        };
    }
    //TODO consider passing only reference
    let records: Vec<AccountSummary> = data
        .drain()
        .map(|record| AccountSummary {
            client: record.0,
            available: record.1.available,
            held: record.1.held,
            total: record.1.total(),
            locked: record.1.is_locked,
        })
        .collect();
    drop(data);
    println!("Total {} records", records.len());
    tx.send(records).unwrap();
    Ok(())
}

//TODO change println to logs or tracing?
fn charge_back(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Account>,
    transaction: crate::Transaction,
) {
    println!("Doing charge back!");
    if let Some(account) = data.get_mut(&transaction.client) {
        account.resolve(transaction);
    }
    // data.insert(message.client, message);
}

fn resolve(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Account>,
    transaction: crate::Transaction,
) {
    println!("Doing resolve!");
    if let Some(account) = data.get_mut(&transaction.client) {
        account.resolve(transaction);
    }
    // data.insert(message.client, message);
}

fn dispute(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Account>,
    transaction: crate::Transaction,
) {
    println!("Doing dispute!");
    if let Some(account) = data.get_mut(&transaction.client) {
        account.dispute(transaction);
    }
    // data.insert(message.client, message);
}

fn withdrawal(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Account>,
    transaction: crate::Transaction,
) {
    println!("Doing withdrawal!");
    if let Some(account) = data.get_mut(&transaction.client) {
        account.withdrawal(transaction);
    } else {
        let client = transaction.client;
        let mut account = Account::new();
        account.withdrawal(transaction);
        data.insert(client, account);
    }
}

fn deposit(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Account>,
    transaction: crate::Transaction,
) {
    println!("Doing deposit!");
    if let Some(account) = data.get_mut(&transaction.client) {
        account.deposit(transaction);
    } else {
        let client = transaction.client;
        let mut account = Account::new();
        account.deposit(transaction);
        data.insert(client, account);
    }
}
