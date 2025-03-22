use crate::event_queue::{EventDefinition, EventPayload};

pub trait Algorithm {
    fn on_event(&mut self, event: EventPayload) -> ();
    fn get_event_listeners(&self) -> Vec<EventDefinition>;
}
