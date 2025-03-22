pub trait AlgorithmAccess {
    fn order(&mut self) -> ();
}

pub trait BacktesterAccess: AlgorithmAccess {
    fn set_balance(&mut self, balance: f64);
}

pub struct Broker {
    balance: f64,
}

impl AlgorithmAccess for Broker {
    fn order(&mut self) -> () {}
}

impl BacktesterAccess for Broker {
    fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
    }
}

impl Broker {
    pub fn new() -> Self {
        Self { balance: 0.0 }
    }
}
