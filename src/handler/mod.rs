use log;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ClientId, Transaction, TransactionId};

pub mod input;
pub mod output;
pub mod transaction;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountSummary {
    pub client: ClientId,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}
pub type DataSource = HashMap<ClientId, Account>;
#[derive(Debug, Clone)]
pub struct Account {
    pub transactions: HashMap<TransactionId, AccountTransaction>,
    pub available: Decimal,
    pub held: Decimal,
    pub is_locked: bool,
}
#[derive(Debug, Clone)]
pub struct AccountTransaction {
    pub amount: Decimal,
    pub tx_type: AccountTransactionType,
    pub state: AccountTransactionState,
}
#[derive(Debug, Clone)]
pub enum AccountTransactionType {
    Deposit,
    Withdrawal,
}
#[derive(Serialize, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum AccountTransactionState {
    Normal,
    Disputed,
    ChargedBack,
}

impl AccountTransaction {
    pub fn new(amount: Decimal, tx_type: AccountTransactionType) -> Self {
        Self {
            amount,
            tx_type,
            state: AccountTransactionState::Normal,
        }
    }
}
impl Account {
    pub fn new() -> Self {
        Account {
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            is_locked: false,
            transactions: HashMap::new(),
        }
    }
    pub fn total(&self) -> Decimal {
        self.available + self.held
    }
    pub fn deposit(&mut self, transaction: Transaction) {
        let amount = transaction.amount.unwrap();
        self.transactions.insert(
            transaction.transaction_id,
            AccountTransaction::new(amount, AccountTransactionType::Deposit),
        );
        self.available += amount;
    }
    pub fn withdrawal(&mut self, transaction: Transaction) {
        let amount = transaction.amount.unwrap();
        if self.available >= amount {
            self.transactions.insert(
                transaction.transaction_id,
                AccountTransaction::new(amount, AccountTransactionType::Withdrawal),
            );
            self.available -= amount;
        } else {
            log::warn!(
                "Could not withdraw. {} available and {amount} requested",
                self.available
            );
        }
    }
    pub fn dispute(&mut self, transaction: Transaction) {
        if let Some(disputed) = self.transactions.get_mut(&transaction.transaction_id) {
            log::info!("Doing dispute {disputed:?}");
            if disputed.state == AccountTransactionState::Disputed {
                //handle this somehow
                return;
            } else {
                //what if there is less than amount?
                match disputed.tx_type {
                    AccountTransactionType::Deposit => {
                        self.available -= disputed.amount;
                        self.held += disputed.amount;
                    }
                    AccountTransactionType::Withdrawal => {
                        self.held += disputed.amount;
                    }
                }
                disputed.state = AccountTransactionState::Disputed;
            }
        }
    }
    pub fn resolve(&mut self, transaction: Transaction) {
        if let Some(to_resolve) = self.transactions.get_mut(&transaction.transaction_id) {
            log::info!("Resolving transaction {to_resolve:?}");
            if to_resolve.state == AccountTransactionState::Disputed {
                match to_resolve.tx_type {
                    AccountTransactionType::Deposit => {
                        self.held -= to_resolve.amount;
                    }
                    AccountTransactionType::Withdrawal => {
                        log::info!("This is data {to_resolve:?}");
                        self.available += to_resolve.amount;
                        self.held -= to_resolve.amount;
                    }
                }
                to_resolve.state = AccountTransactionState::Normal;
            } else {
                //handle this somehow
                return;
            }
        }
    }
    pub fn charge_back(&mut self, transaction: Transaction) {
        log::info!("Charging back transaction");
        if let Some(to_charge_back) = self.transactions.get_mut(&transaction.transaction_id) {
            if to_charge_back.state == AccountTransactionState::Disputed {
                log::info!("ACCOUNT LOCKED {to_charge_back:?}");
                self.held -= to_charge_back.amount;
                self.is_locked = true;
            } else {
                log::info!("Transaction is not in Disputed State but in {to_charge_back:?}")
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use rust_decimal::Decimal;

    use crate::{handler::Account, Transaction, TransactionType};
    #[test]
    fn typical_scenario() {
        let client = 1;
        let deposit = Transaction {
            tx_type: TransactionType::Deposit,
            client_id: client,
            transaction_id: 1,
            amount: Some(Decimal::ONE_HUNDRED),
        };
        let mut account = Account::new();
        account.deposit(deposit);
        assert_eq!(account.total(), Decimal::ONE_HUNDRED);
        assert_eq!(account.available, Decimal::ONE_HUNDRED);
        let withdraw = Transaction {
            tx_type: TransactionType::Withdrawal,
            client_id: client,
            transaction_id: 2,
            amount: Some(Decimal::ONE_HUNDRED),
        };
        account.withdrawal(withdraw);
        assert_eq!(account.available, Decimal::ZERO);
        assert_eq!(account.total(), Decimal::ZERO);
    }

    #[test]
    fn dispute_with_resolve() {
        let client_id = 1;
        let deposit = Transaction {
            tx_type: TransactionType::Deposit,
            client_id,
            transaction_id: 1,
            amount: Some(Decimal::ONE_HUNDRED),
        };
        let mut account = Account::new();
        account.deposit(deposit);
        assert_eq!(account.available, Decimal::ONE_HUNDRED);

        let withdraw = Transaction {
            tx_type: TransactionType::Withdrawal,
            client_id,
            transaction_id: 2,
            amount: Some(Decimal::new(70, 0)),
        };
        account.withdrawal(withdraw);
        assert_eq!(account.available, Decimal::new(30, 0));

        let dispute = Transaction {
            tx_type: TransactionType::Dispute,
            client_id,
            transaction_id: 2,
            amount: None,
        };
        account.dispute(dispute);
        assert_eq!(account.available, Decimal::new(30, 0));
        assert_eq!(account.held, Decimal::new(70, 0));
        assert_eq!(account.total(), Decimal::new(100, 0));

        let resolve = Transaction {
            tx_type: TransactionType::Resolve,
            client_id,
            transaction_id: 2,
            amount: None,
        };
        account.resolve(resolve);
        assert_eq!(account.available, Decimal::new(100, 0));
        assert_eq!(account.held, Decimal::new(0, 0));
        assert_eq!(account.total(), Decimal::new(100, 0));
    }

    #[test]
    fn dispute_with_charge_back() {
        let client = 1;
        let deposit = Transaction {
            tx_type: TransactionType::Deposit,
            client_id: client,
            transaction_id: 1,
            amount: Some(Decimal::ONE_HUNDRED),
        };
        let mut account = Account::new();
        account.deposit(deposit);
        assert_eq!(account.available, Decimal::ONE_HUNDRED);

        let withdraw = Transaction {
            tx_type: TransactionType::Withdrawal,
            client_id: client,
            transaction_id: 2,
            amount: Some(Decimal::new(70, 0)),
        };
        account.withdrawal(withdraw);
        assert_eq!(account.available, Decimal::new(30, 0));

        let dispute = Transaction {
            tx_type: TransactionType::Dispute,
            client_id: client,
            transaction_id: 2,
            amount: None,
        };
        account.dispute(dispute);
        assert_eq!(account.available, Decimal::new(30, 0));
        assert_eq!(account.held, Decimal::new(70, 0));
        assert_eq!(account.total(), Decimal::new(100, 0));

        let charge_back = Transaction {
            tx_type: TransactionType::ChargeBack,
            client_id: client,
            transaction_id: 2,
            amount: None,
        };
        account.charge_back(charge_back);
        assert_eq!(account.available, Decimal::new(30, 0));
        assert_eq!(account.held, Decimal::new(0, 0));
        assert_eq!(account.total(), Decimal::new(30, 0));
        assert!(account.is_locked);
    }
}
