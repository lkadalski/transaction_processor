use crate::DataSource;
use anyhow::Result;
use tokio::sync::{mpsc::Receiver, oneshot};

pub async fn handle(input: Receiver<crate::Transaction>) -> Result<oneshot::Receiver<DataSource>> {
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
    tx: oneshot::Sender<DataSource>,
) -> Result<()> {
    let mut data = DataSource::new();
    while let Some(message) = queue.recv().await {
        match message.ttype {
            crate::TransactionType::Deposit => deposit(&mut data, message),
            crate::TransactionType::Withdrawal => withdrawal(&mut data, message),
            crate::TransactionType::Dispute => dispute(&mut data, message),
            crate::TransactionType::Resolve => resolve(&mut data, message),
            crate::TransactionType::ChargeBack => charge_back(&mut data, message),
        };
    }
    tx.send(data).unwrap();
    Ok(())
}

fn charge_back(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Transaction>,
    message: crate::Transaction,
) {
    println!("Doing charge back!");
    data.insert(message.client, message);
}

fn resolve(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Transaction>,
    message: crate::Transaction,
) {
    println!("Doing resolve!");
    data.insert(message.client, message);
}

fn dispute(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Transaction>,
    message: crate::Transaction,
) {
    println!("Doing dispute!");
    data.insert(message.client, message);
}

fn withdrawal(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Transaction>,
    message: crate::Transaction,
) {
    println!("Doing withdrawal!");
    data.insert(message.client, message);
}

fn deposit(
    data: &mut std::collections::HashMap<crate::ClientId, crate::Transaction>,
    message: crate::Transaction,
) {
    println!("Doing deposit!");
    data.insert(message.client, message);
}
