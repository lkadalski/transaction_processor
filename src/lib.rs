use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Uniquely identifies a Deposit or Withdraw transaction.
pub struct TransactionId(pub u32);
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Uniquely identifies a Client.
pub struct ClientId(pub u32);
/// Represents an input transaction line in the input csv.
#[derive(Debug, PartialEq)]
pub enum Transaction {
    Deposit {
        client: ClientId,
        tx: TransactionId,
        amount: f64,
    },
    Withdrawal {
        client: ClientId,
        tx: TransactionId,
        amount: f64,
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
