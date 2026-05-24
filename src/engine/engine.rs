use chrono::{DateTime, Utc};
use rust_decimal::{Decimal, dec};
use uuid::Uuid;

use crate::{
    engine::config::EngineConfig,
    types::{
        fill::Fill,
        order::{api::create_order::CreateOrderRequest, order::Order, side::Side, status::Status},
        position::Position,
    },
};
use std::collections::{BTreeMap, HashMap, VecDeque};
pub struct Engine {
    pub engine_config: EngineConfig,
    pub bids: BTreeMap<Decimal, Vec<Order>>,
    pub asks: BTreeMap<Decimal, Vec<Order>>,
    pub liquidation_index: BTreeMap<Decimal, Vec<Uuid>>,
    pub positions: HashMap<Uuid, Position>,
    pub index_price: Decimal,
    pub mark_price: Decimal,
    pub last_10_index_prices: VecDeque<Decimal>,
    pub last_funding_time: DateTime<Utc>,
    pub insurance_fund: Decimal,
    pub trades: HashMap<Uuid, Fill>,
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
            trades: HashMap::new(),
        }
    }

    pub fn place_order(&mut self, order: CreateOrderRequest) {
        //LONG = BID
        //SHORT = ASK
        let order_id: Uuid = Uuid::new_v4();
        let timestamp = Utc::now();
        let mut status: Status = Status::Open;

        match order.side {
            Side::Long => {
                let (executed_quantity, fills) = self.match_asks(&order, order_id);
                if executed_quantity == dec!(0) {
                    status = Status::Pending
                } else if executed_quantity < order.quantity {
                    status = Status::PartialFilled(executed_quantity)
                } else if executed_quantity == order.quantity {
                    status = Status::Filled
                }
                if executed_quantity > dec!(0) {
                    let position_id = Uuid::new_v4();

                    let liquidation_price = self.index_price
                        - (dec!(1) - dec!(1) / order.leverage
                            + self.engine_config.maintainance_margin_rate);

                    let position: Position = Position {
                        position_id,
                        asset: order.asset.clone(),
                        user_id: order.user_id,
                        side: Side::Long,
                        status: status,
                        entry_price: order.price,
                        unrealised_pnl: dec!(0),
                        liquidation_price,
                        opened_at: self.index_price,
                        quantity: executed_quantity,
                        leverage: order.leverage,
                    };

                    self.positions.insert(position_id, position);
                    self.liquidation_index
                        .entry(liquidation_price)
                        .or_default()
                        .push(position_id);
                }

                if executed_quantity < order.quantity {
                    let order = Order {
                        asset: order.asset,
                        created_at: timestamp,
                        filled_quantity: executed_quantity,
                        leverage: order.leverage,
                        margin: order.margin,
                        order_id,
                        price: order.price,
                        quantity: order.quantity,
                        side: order.side,
                        status,
                        user_id: order.user_id,
                    };
                    self.bids.entry(order.price).or_default().push(order);
                }
            }
            Side::Short => {
                let (executed_quantity, fills) = self.match_bids(order, order_id);
            }
        }
    }

    pub fn match_bids(
        &mut self,
        order: CreateOrderRequest,
        order_id: Uuid,
    ) -> (Decimal, Vec<Fill>) {
        //Order{Ask, price:101, qty:10}
        //Willing to sell 10 at 101
        let mut fills = Vec::<Fill>::new();
        let mut executed_quantity = dec!(0);

        //bids - [99, 99.10, 99.20, 99.30 .....100]
        //bids needs rev()
        //bids - [100, 99.90, 99.80, ........., 99]
        //the best bid is at 100
        //if the order.price > price [101 > 100],
        //break(no need to check further)

        for (price, bids) in self.bids.iter_mut().rev() {
            if order.price > *price {
                break;
            }
            for bid in bids.iter_mut() {
                if order.price <= bid.price && executed_quantity < order.quantity {
                    let quantity_left = order.quantity - executed_quantity;
                    let quantity_matched =
                        std::cmp::min(quantity_left, bid.quantity - bid.filled_quantity);
                    executed_quantity += quantity_matched;
                    bid.filled_quantity += quantity_matched;
                    let trade_id = Uuid::new_v4();
                    let fill = Fill {
                        maker_order_id: bid.order_id,
                        taker_order_id: order_id,
                        price: bid.price,
                        quantity: quantity_matched,
                        trade_id,
                    };
                    fills.push(fill);
                }
            }
            bids.retain(|bid| bid.filled_quantity < bid.quantity);
        }
        (executed_quantity, fills)
    }
    pub fn match_asks(
        &mut self,
        order: &CreateOrderRequest,
        order_id: Uuid,
    ) -> (Decimal, Vec<Fill>) {
        //Bid{price:100, qty:10}
        //willing to buy 10 at 100

        //asks - [100.10, 100.20, .....101]

        let mut executed_quantity = dec!(0);
        let mut fills = Vec::<Fill>::new();

        for (price, asks) in self.asks.iter_mut() {
            if order.price < *price {
                break;
            }
            for ask in asks.iter_mut() {
                if order.price >= ask.price && executed_quantity < order.quantity {
                    let quantity_left = order.quantity - executed_quantity;
                    let matched_quantity =
                        std::cmp::min(quantity_left, ask.quantity - ask.filled_quantity);
                    executed_quantity += matched_quantity;
                    ask.filled_quantity += matched_quantity;
                    let trade_id = Uuid::new_v4();
                    let fill = Fill {
                        maker_order_id: ask.order_id,
                        price: ask.price,
                        quantity: matched_quantity,
                        taker_order_id: order_id,
                        trade_id: trade_id,
                    };
                    fills.push(fill);
                }
            }
            asks.retain(|ask: &Order| ask.filled_quantity < ask.quantity);
        }
        (executed_quantity, fills)
    }
}
