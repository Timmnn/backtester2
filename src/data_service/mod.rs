pub trait DataService {
    fn new() -> Self;
}

pub struct BacktestDataService {}

impl DataService for BacktestDataService {
    fn new() -> Self {
        Self {}
    }
}
