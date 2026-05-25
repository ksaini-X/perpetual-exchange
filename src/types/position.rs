use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::order::{side::Side, status::Status};

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub position_id: Uuid,
    pub user_id: Uuid,
    pub asset: String,

    pub side: Side,
    pub status: Status,
    pub margin: Decimal,
    pub entry_price: Decimal,
    pub unrealised_pnl: Decimal,
    pub liquidation_price: Decimal,

    pub created_at: DateTime<Utc>,
    pub quantity: Decimal,

    pub leverage: Decimal,
}
