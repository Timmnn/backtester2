use crate::backtest_data_service::DatasetSubscription;
use crate::event_queue::{EventDefinition, EventPayload};
use crate::runtime::Runtime;

pub trait Algorithm {
    fn on_event(&mut self, event: EventPayload, runtime: &mut Runtime) -> ();
    fn get_event_listeners(&self) -> Vec<EventDefinition>;
    fn get_dataset_subscriptions(&self) -> Vec<DatasetSubscription>;
}
