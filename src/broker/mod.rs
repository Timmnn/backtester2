use rand::Rng;
use std::collections::HashMap; // 0.8.5

pub trait AlgorithmAccess {
    fn order(&mut self, ticker: String, quantity: f64) -> ();
    fn get_holdings(&self) -> &HashMap<String, f64>;
    fn get_balance(&self) -> f64;
}

pub trait BacktesterAccess: AlgorithmAccess {
    fn set_balance(&mut self, balance: f64);
}

pub struct Broker {
    balance: f64,
    holdings: HashMap<String, f64>,
}

impl AlgorithmAccess for Broker {
    fn order(&mut self, ticker: String, quantity: f64) -> () {
        //TODO: implement data fetching
        let mut rng = rand::rng();

        let price = rng.random_range(10.0..20.0);

        let new_balance = self.balance - price * quantity;

        if new_balance >= 0.0 {
            let current_holdings = *self.holdings.get(&ticker).unwrap_or(&0.0);
            let new_holdings = current_holdings + quantity;
            self.holdings.insert(ticker, new_holdings);
            self.balance = new_balance;
        }
    }
    fn get_holdings(&self) -> &HashMap<String, f64> {
        &self.holdings
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

impl BacktesterAccess for Broker {
    fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
    }
}

impl Broker {
    pub fn new() -> Self {
        Self {
            balance: 0.0,
            holdings: HashMap::new(),
        }
    }
}
