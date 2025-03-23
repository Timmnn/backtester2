use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeLog {
    pub ticker: String,
    pub price: f64,
    pub quantity: f64,
    pub time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceLog {
    pub balance: f64,
    pub time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BacktestLog {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub trades: Vec<TradeLog>,
    pub balance_history: Vec<BalanceLog>,
}
