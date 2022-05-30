use std::{collections::HashMap, sync::Arc};

use rust_decimal::Decimal;
use serde::Serialize;

pub mod handler;
type DataSource = Arc<HashMap<ClientId, Transaction>>;
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
#[derive(Debug, Clone, serde::Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Transaction {
    Deposit {
        client: ClientId,
        tx: TransactionId,
        #[serde(with = "rust_decimal::serde::float")]
        amount: Decimal,
    },
    Withdrawal {
        client: ClientId,
        tx: TransactionId,
        #[serde(with = "rust_decimal::serde::float")]
        amount: Decimal,
    },
    Dispute {
        client: ClientId,
        tx: TransactionId,
    },
    Resolve {
        client: ClientId,
        tx: TransactionId,
    },
    ChargeBack {
        client: ClientId,
        tx: TransactionId,
    },
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
