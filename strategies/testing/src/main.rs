use backtester::{
    algorithm::Algorithm,
    backtester::Backtester,
    event_queue::{EventDefinition, EventPayload},
    logging::{LogLevel, Logger},
};
use chrono::NaiveTime;

struct Algo1 {
    state: usize,
}
impl Algorithm for Algo1 {
    fn on_event(&mut self, event: EventPayload) -> () {
        Logger::log(LogLevel::Info, "Hi");
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
    fn on_event(&mut self, _event: EventPayload) -> () {}
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
