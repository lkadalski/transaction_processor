use super::AccountSummary;
use crate::{handler::Account, handler::DataSource, ClientId, Transaction};
use anyhow::{bail, Result};
use tokio::sync::{mpsc::Receiver, oneshot};

pub async fn handle(input: Receiver<crate::Transaction>) -> oneshot::Receiver<Vec<AccountSummary>> {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        if let Err(err) = process(input, tx).await {
            log::error!("Could not process transaction due {err}");
            std::process::exit(1)
        }
    });
    rx
}

async fn process(
    mut queue: Receiver<crate::Transaction>,
    tx: oneshot::Sender<Vec<AccountSummary>>,
) -> Result<()> {
    let mut data = DataSource::new();
    while let Some(message) = queue.recv().await {
        match message.tx_type {
            crate::TransactionType::Deposit => deposit(&mut data, &message),
            crate::TransactionType::Withdrawal => withdrawal(&mut data, &message),
            crate::TransactionType::Dispute => dispute(&mut data, &message),
            crate::TransactionType::Resolve => resolve(&mut data, &message),
            crate::TransactionType::ChargeBack => charge_back(&mut data, &message),
        };
    }
    let records: Vec<AccountSummary> = data
        .drain()
        .map(|record| AccountSummary {
            client: record.0,
            available: record.1.available.round_dp(4),
            held: record.1.held.round_dp(4),
            total: record.1.total().round_dp(4),
            locked: record.1.is_locked,
        })
        .collect();
    log::info!("Total {} records", records.len());

    // Possibly handle those data somehow differently
    if let Err(_err) = tx.send(records) {
        bail!("Could not push records to output handler")
    }
    Ok(())
}

fn charge_back(data: &mut std::collections::HashMap<ClientId, Account>, transaction: &Transaction) {
    log::info!("Doing charge back!");
    if let Some(account) = data.get_mut(&transaction.client_id) {
        account.charge_back(transaction);
    } else {
        log::error!("There is not client with {transaction:?}");
    }
}

fn resolve(data: &mut std::collections::HashMap<ClientId, Account>, transaction: &Transaction) {
    log::info!("Doing resolve!");
    if let Some(account) = data.get_mut(&transaction.client_id) {
        account.resolve(transaction);
    }
}

fn dispute(data: &mut std::collections::HashMap<ClientId, Account>, transaction: &Transaction) {
    log::info!("Doing dispute!");
    if let Some(account) = data.get_mut(&transaction.client_id) {
        account.dispute(transaction);
    }
}

fn withdrawal(data: &mut std::collections::HashMap<ClientId, Account>, transaction: &Transaction) {
    log::info!("Doing withdrawal!");
    if let Some(account) = data.get_mut(&transaction.client_id) {
        account.withdrawal(transaction);
    } else {
        let client = transaction.client_id;
        let mut account = Account::default();
        account.withdrawal(transaction);
        data.insert(client, account);
    }
}

fn deposit(data: &mut std::collections::HashMap<ClientId, Account>, transaction: &Transaction) {
    log::info!("Doing deposit!");
    if let Some(account) = data.get_mut(&transaction.client_id) {
        account.deposit(transaction);
    } else {
        let client = transaction.client_id;
        let mut account = Account::default();
        account.deposit(transaction);
        data.insert(client, account);
    }
}
