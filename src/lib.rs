use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub mod handler;
pub type DataSource = HashMap<ClientId, Transaction>;
#[derive(
    Serialize, Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
/// Uniquely identifies a Deposit or Withdraw transaction.
pub struct TransactionId(pub u32);
#[derive(
    Serialize, Debug, serde::Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
/// Uniquely identifies a Client.
pub struct ClientId(pub u16);
/// Represents an input transaction line in the input csv.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Transaction {
    #[serde(rename = "type")]
    pub ttype: TransactionType,
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Option<Decimal>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
/// Represents all the different types of transactions.
///
/// Used for parsing and serializing.
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    ChargeBack,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
