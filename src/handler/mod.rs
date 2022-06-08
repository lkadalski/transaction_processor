use rust_decimal::Decimal;
use serde::Serialize;

use crate::ClientId;

pub mod input;
pub mod output;
pub mod transaction;

#[derive(Serialize, Debug)]
pub struct AccountSummary {
    #[serde(flatten)]
    pub client: ClientId,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}
