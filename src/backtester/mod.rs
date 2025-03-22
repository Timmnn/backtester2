use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

use crate::{
    algorithm::{self, Algorithm},
    backtest_data_service::DataService,
    event_queue::{EventDefinition, EventPayload, EventQueue, EventQueuePosition},
};

pub struct Backtester {
    algorithms: Vec<Box<dyn Algorithm>>,
    event_queue: EventQueue,
    data_service: DataService,
}

const PREFILL_QUEUE_DAYS: u32 = 5;

impl Backtester {
    pub fn new(mut algorithms: Vec<Box<dyn Algorithm>>) -> Self {
        let event_queue = EventQueue::new();
        let data_service = DataService::new();
        let event_listeners: Vec<Vec<EventDefinition>> = algorithms
            .iter_mut()
            .map(|algorithm| algorithm.get_event_listeners())
            .collect();

        Self {
            algorithms,
            event_queue,
            data_service,
        }
    }
    pub fn run(&mut self) {
        let start_date = NaiveDate::from_ymd_opt(2020, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_date = NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let mut current_date = start_date;
        while current_date < end_date {
            println!("Current Time is: {}", current_date);
            println!("Event_Queue Length: {}", self.event_queue.len());

            self.enqueue_predictable_events(
                current_date,
                current_date
                    .checked_add_days(chrono::Days::new(PREFILL_QUEUE_DAYS as u64))
                    .unwrap(),
            );

            let event = self.event_queue.consume();

            match event {
                Some(event) => {
                    current_date = event.time;

                    self.algorithms[0].on_event(event.event);
                }
                None => {}
            }
        }
    }

    fn enqueue_predictable_events(&mut self, from: NaiveDateTime, till: NaiveDateTime) {
        for algorithm in self.algorithms.iter() {
            for event_definition in algorithm.get_event_listeners() {
                match event_definition {
                    EventDefinition::SpecificTime(time) => {
                        let mut current =
                            NaiveDate::from_ymd_opt(from.year(), from.month(), from.day())
                                .unwrap()
                                .and_hms_opt(time.hour(), time.minute(), time.second())
                                .unwrap();

                        if current <= from {
                            current = current.checked_add_days(chrono::Days::new(1)).unwrap();
                        }

                        while current < till {
                            let event_pos = EventQueuePosition {
                                time: current,
                                event: EventPayload::SpecificTime(current),
                            };

                            if !self.event_queue.contains(&event_pos) {
                                println!("{}", event_pos.time);
                                self.event_queue.enqueue(event_pos);
                            }

                            current = current.checked_add_days(chrono::Days::new(1)).unwrap();
                        }
                    }
                    _ => todo!("This Event is not yet implemented."),
                }
            }
        }
    }
}
