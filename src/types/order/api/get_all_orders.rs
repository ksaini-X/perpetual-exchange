use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct CreateOrderRequest {
    asset: String,
}

#[derive(Deserialize, Debug)]
pub enum CreateOrderResponseStatus {
    Accepted,
    Rejected,
}
