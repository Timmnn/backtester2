use crate::{
    algorithm::{Algorithm, EventListener},
    data_service::{BacktestDataService, DataService},
    time_service::BacktestTimeService,
};

pub struct Backtest {
    time_service: BacktestTimeService,
    data_service: BacktestDataService,
    strategies: Vec<Box<dyn Algorithm<'_>>>,
    strategy_event_listeners: Vec<EventListener>,
}

impl Backtest {
    pub fn new() -> Self {
        let time_service = BacktestTimeService::new();
        let data_service = BacktestDataService::new();
        let strategies = vec![];
        let strategy_event_listeners = vec![];

        Self {
            time_service,
            data_service,
            strategies,
            strategy_event_listeners,
        }
    }

    pub fn add_strategy(&mut self, strategy: impl Algorithm + 'static) {
        let event_listeners = strategy.register_events();

        self.strategy_event_listeners.extend(event_listeners);
    }

    pub fn run(&mut self, start_time: chrono::NaiveDateTime) {
        self.time_service.set_time(start_time);
    }
}
