use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::order::{side::Side, status::Status};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub asset: String,

    pub price: Decimal,
    pub leverage: Decimal,
    pub quantity: Decimal,
    pub margin: Decimal,
    pub filled_quantity: Decimal,
    pub status: Status,
    pub side: Side,

    pub created_at: DateTime<Utc>,
}
