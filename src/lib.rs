use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub mod handler;
pub type DataSource = HashMap<ClientId, Data>;
#[derive(
    Serialize, Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Data {
    pub client_id: ClientId,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}
impl Data {
    pub fn new(ammount: Decimal, client_id: ClientId) -> Self {
        Data {
            available: ammount,
            held: Decimal::new(0, 0),
            total: ammount,
            locked: false,
            client_id,
        }
    }
    pub fn deposit(&mut self, ammount: Decimal) {
        self.available += ammount;
        self.total += ammount;
    }
    pub fn withdrawal(&mut self, ammount: Decimal) {
        if self.available > ammount && self.total > ammount {
            self.available -= ammount;
            self.total -= ammount;
        } else {
            eprintln!(
                "Could not withdraw. {} available and {ammount} requested",
                self.available
            );
        }
    }
    pub fn dispute(&mut self) {}
    pub fn resolve(&mut self) {}
    pub fn charge_back(&mut self) {}
}
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
