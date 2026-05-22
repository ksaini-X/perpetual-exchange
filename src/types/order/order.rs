use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::order::{order_type::OrderType, side::Side, status::Status};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub user_id: Uuid,
    pub asset: String,

    pub price: Option<Decimal>,
    pub leverage: Decimal,
    pub quantity: Decimal,
    pub margin: Decimal,

    pub order_type: OrderType,

    pub status: Status,
    pub side: Side,

    pub created_at: DateTime<Utc>,
}
