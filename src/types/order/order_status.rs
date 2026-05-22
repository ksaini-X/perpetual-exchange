use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OrderStatus {
    Filled,
    PartialFilled,
    Open,
    Closed,
}
