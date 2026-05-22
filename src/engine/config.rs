use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize)]
pub struct EngineConfig {
    pub asset: String,
    pub max_leverage: Decimal,
    pub funding_interval: Decimal,
    pub max_positions: u8,
    pub maintainance_margin_rate: Decimal,
}
