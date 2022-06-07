use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod handler;
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
pub type DataSource = HashMap<ClientId, Account>;
#[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
pub struct Account {
    pub client_id: ClientId,
    pub transactions: HashMap<TransactionId, AccountTransaction>,
    pub available: Decimal,
    pub held: Decimal,
    pub locked: bool,
}
#[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
pub struct AccountTransaction {
    pub amount: Decimal,
    pub state: AccountTransactionState,
}
#[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
pub enum NormalTransactionType {
    Deposit,
    Withdrawal,
}
#[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
pub enum AccountTransactionState {
    Normal(NormalTransactionType),
    Disputed,
    ChargedBack,
}
impl AccountTransaction {
    pub fn new(amount: Decimal) -> Self {
        Self {
            amount,
            state: AccountTransactionState::Normal(NormalTransactionType::Deposit),
        }
    }
}
impl Account {
    pub fn new(client_id: ClientId) -> Self {
        Account {
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            locked: false,
            client_id,
            transactions: HashMap::new(),
        }
    }
    pub fn total(&self) -> Decimal {
        self.available + self.held
    }
    pub fn deposit(&mut self, transaction: Transaction) {
        let amount = transaction.amount.unwrap();
        self.transactions
            .insert(transaction.tx, AccountTransaction::new(amount));
        self.available += amount;
    }
    pub fn withdrawal(&mut self, transaction: Transaction) {
        let amount = transaction.amount.unwrap();
        if self.available >= amount {
            self.transactions
                .insert(transaction.tx, AccountTransaction::new(amount));
            self.available -= amount;
        } else {
            eprintln!(
                "Could not withdraw. {} available and {amount} requested",
                self.available
            );
        }
    }
    pub fn dispute(&mut self, transaction: Transaction) {
        if let Some(disputed) = self.transactions.get_mut(&transaction.tx) {
            if disputed.state == AccountTransactionState::Disputed {
                //handle this somehow
                return;
            } else {
                //what if there is less than amount?
                self.available -= disputed.amount;
                self.held += disputed.amount;
            }
        }
    }
    pub fn resolve(&mut self, transaction: Transaction) {
        if let Some(to_resolve) = self.transactions.get_mut(&transaction.tx) {
            if to_resolve.state == AccountTransactionState::Disputed {
                self.available += to_resolve.amount;
                self.held -= to_resolve.amount;
                to_resolve.state = AccountTransactionState::Normal;
            } else {
                //handle this somehow
                return;
            }
        }
    }
    pub fn charge_back(&mut self, transaction: Transaction) {
        if let Some(to_charge_back) = self.transactions.get_mut(&transaction.tx) {
            if to_charge_back.state == AccountTransactionState::Disputed {
                self.held -= to_charge_back.amount;
                self.locked = true;
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use rust_decimal::Decimal;

    use crate::{Account, ClientId, Transaction, TransactionId, TransactionType};
    #[test]
    fn typical_scenario() {
        let deposit = Transaction {
            ttype: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(Decimal::ONE_HUNDRED),
        };
        let mut account = Account::new(ClientId(1));
        account.deposit(deposit);
        assert_eq!(account.total(), Decimal::ONE_HUNDRED);
        assert_eq!(account.available, Decimal::ONE_HUNDRED);
        let withdraw = Transaction {
            ttype: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: Some(Decimal::ONE_HUNDRED),
        };
        account.withdrawal(withdraw);
        assert_eq!(account.available, Decimal::ZERO);
        assert_eq!(account.total(), Decimal::ZERO);
    }
    #[test]
    fn dispute_with_resolve() {
        let deposit = Transaction {
            ttype: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(Decimal::ONE_HUNDRED),
        };
        let mut account = Account::new(ClientId(1));
        account.deposit(deposit);
        assert_eq!(account.available, Decimal::ONE_HUNDRED);
        let withdraw = Transaction {
            ttype: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: Some(Decimal::new(7000, 2)),
        };
        account.withdrawal(withdraw);
        assert_eq!(account.available, Decimal::new(3000, 2));
        let dispute = Transaction {
            ttype: TransactionType::Dispute,
            client: ClientId(1),
            tx: TransactionId(2),
            amount: None,
        };
        account.dispute(dispute);
        assert_eq!(account.available, Decimal::new(3000, 2));
    }
}
