use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct CreateOrderRequest {
    user_id: Uuid,
    asset: String,
    order_type: CreateOrderType,
    quantity: Decimal,
    leverage: Decimal,
    margin: Decimal,
    price: Option<Decimal>,
}

#[derive(Serialize, Debug)]
pub enum CreateOrderType {
    Market,
    Limit,
}

#[derive(Deserialize, Debug)]
pub struct CreateOrderResponse {
    pub order_id: Uuid,
    pub status: CreateOrderResponseStatus,
}

#[derive(Deserialize, Debug)]
pub enum CreateOrderResponseStatus {
    Accepted,
    Rejected,
}
