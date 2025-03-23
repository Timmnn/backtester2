use backtester::{
    algorithm::Algorithm,
    backtester::Backtester,
    event_queue::{EventDefinition, EventPayload},
    logging::{LogLevel, Logger},
    runtime::Runtime,
};
use chrono::NaiveTime;

struct Algo1 {
    state: usize,
}
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
}

struct Algo2 {
    state: String,
}
impl Algorithm for Algo2 {
    fn on_event(&mut self, _event: EventPayload, runtime: &mut Runtime) -> () {}
    fn get_event_listeners(&self) -> Vec<EventDefinition> {
        vec![]
    }
}

fn main() {
    let mut backtester = Backtester::new(vec![
        Box::new(Algo1 { state: 1 }),
        Box::new(Algo2 {
            state: "".to_string(),
        }),
    ]);

    backtester.run();
}
