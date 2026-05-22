use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::order::{side::Side, status::Status};

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub position_id: Uuid,
    pub asset: String,
    pub user_id: Uuid,
    pub side: Side,
    pub status: Status,
    pub entry_price: Decimal,
    pub unrealised_pnl: Decimal,
    pub liqudation_price: Decimal,
    pub opened_at: Decimal,
    pub quantity: Decimal,
    pub leverage: Decimal,
}
