use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    status: Status,
    pub data: HashMap<Ticker, Data>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    timestamp: String,
    error_code: u32,
    error_message: Option<String>,
    elapsed: u32,
    credit_count: u32,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Ticker {
    BTC,
    ETH,
    XRP,
    BCH,
    LTC,
    XLM,
    ETC,
    DOT,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Quote {
    USD { price: f64, market_cap: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    id: u32,
    name: String,
    pub quote: Quote,
}
