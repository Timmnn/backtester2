use std::path::Path;
use std::path::PathBuf;

use backtester::backtest_data_service::DatasetSubscription;
use backtester::{
    algorithm::Algorithm,
    backtester::Backtester,
    event_queue::{EventDefinition, EventPayload},
    logging::{LogLevel, Logger},
    runtime::Runtime,
};
use chrono::NaiveTime;

struct Algo1 {}

impl Algorithm for Algo1 {
    fn on_event(&mut self, event: EventPayload, runtime: &mut Runtime) -> () {
        Logger::log(LogLevel::Info, "Processing Event");
        let mut broker = runtime.broker.borrow_mut();

        broker.order("SPY".to_string(), 1.0);

        Logger::log(
            LogLevel::Info,
            format!(
                "Current Holdings: {:?}. Current Balance {}",
                broker.get_holdings(),
                broker.get_balance()
            ),
        );
    }

    fn get_event_listeners(&self) -> Vec<EventDefinition> {
        vec![
            EventDefinition::SpecificTime(NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
            EventDefinition::SpecificTime(NaiveTime::from_hms_opt(9, 2, 0).unwrap()),
        ]
    }

    fn get_dataset_subscriptions(&self) -> Vec<DatasetSubscription> {
        vec![]
    }
}

fn main() {
    let mut backtester = Backtester::new(
        vec![Box::new(Algo1 {})],
        PathBuf::from("/Users/timmnicolaizik/Dev/vireo/vireo-data/data/"),
    );

    backtester.run();
}
