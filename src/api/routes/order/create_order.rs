use axum::Json;

use crate::types::order::api::create_order::{CreateOrderRequest, CreateOrderResponse};

async fn create_order(Json(payload): Json<CreateOrderRequest>) {}
