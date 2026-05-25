use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::order::side::Side;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderRequest {
    pub user_id: Uuid,
    pub asset: String,

    pub leverage: Decimal,
    pub margin: Decimal,

    pub side: Side,
    pub price: Decimal,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateOrderResponse {
    pub order_id: Uuid,
    pub status: CreateOrderResponseStatus,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum CreateOrderResponseStatus {
    Accepted,
    Rejected,
}
