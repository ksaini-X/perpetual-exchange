use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderRequest {
    user_id: Uuid,
    asset: String,

    order_type: CreateOrderType,

    quantity: Decimal,
    leverage: Decimal,
    margin: Decimal,

    price: Option<Decimal>,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum CreateOrderType {
    Market,
    Limit,
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
