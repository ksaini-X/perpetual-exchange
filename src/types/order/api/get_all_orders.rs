use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CreateOrderRequest {
    asset: String,
}

#[derive(Deserialize, Debug)]
pub enum CreateOrderResponseStatus {
    Accepted,
    Rejected,
}
