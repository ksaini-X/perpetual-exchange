use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    Pending,
    Filled,
    PartialFilled(Decimal),
    Open,
    Closed,
}
