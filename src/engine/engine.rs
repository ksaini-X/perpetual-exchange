use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    engine::config::EngineConfig,
    types::{
        order::{api::create_order::CreateOrderRequest, order::Order},
        position::Position,
    },
};
use std::collections::{BTreeMap, HashMap, VecDeque};
pub struct Engine {
    pub engine_config: EngineConfig,
    pub bids: BTreeMap<Decimal, VecDeque<Order>>,
    pub asks: BTreeMap<Decimal, VecDeque<Order>>,
    pub liquidation_index: BTreeMap<Decimal, Vec<Uuid>>,
    pub positions: HashMap<Uuid, Position>,
    pub index_price: Decimal,
    pub mark_price: Decimal,
    pub last_10_index_prices: VecDeque<Decimal>,
    pub last_funding_time: DateTime<Utc>,
    pub insurance_fund: Decimal,
}

impl Engine {
    pub fn new(engine_config: EngineConfig) -> Self {
        Self {
            engine_config,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            liquidation_index: BTreeMap::new(),
            positions: HashMap::new(),
            index_price: Decimal::from(0),
            mark_price: Decimal::from(0),
            last_10_index_prices: VecDeque::new(),
            last_funding_time: Utc::now(),
            insurance_fund: Decimal::from(0),
        }
    }

    pub fn place_order(order_input: CreateOrderRequest) {
        let order_id = uuid::Uuid::new_v4();
    }
}
