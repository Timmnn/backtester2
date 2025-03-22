use std::hash;

use chrono::{NaiveDateTime, NaiveTime};

use crate::backtest_data_service::Quote;

macro_rules! generate_enums {
    // Match a list of tuples with types
    ([$(($name:ident, $a:ty, $b:ty)),*]) => {
        // Generate enum A
        pub enum EventDefinition {
            $($name($a)),*
        }

        // Generate enum B
        #[derive(PartialEq, Debug)]
        pub enum EventPayload {
            $($name($b)),*
        }
    };
}

// Invoke the macro with the provided list of tuples
generate_enums!([(SpecificTime, NaiveTime, NaiveDateTime), (Data, (), Quote)]);

pub struct EventQueue {
    events: Vec<EventQueuePosition>,
}

#[derive(PartialEq)]
pub struct EventQueuePosition {
    pub event: EventPayload,
    pub time: NaiveDateTime,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn enqueue(&mut self, item: EventQueuePosition) {
        self.events.push(item);
        // Sort events by time (oldest first) after insertion
        self.events.sort_by(|a, b| a.time.cmp(&b.time));
    }

    pub fn consume(&mut self) -> Option<EventQueuePosition> {
        Some(self.events.remove(0))
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn contains(&self, event_position: &EventQueuePosition) -> bool {
        self.events.contains(event_position)
    }
}
