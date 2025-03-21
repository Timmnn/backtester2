use crate::runtime::Runtime;

pub enum EventListener {
    SpecificTime(u64, Box<dyn Fn(&Runtime) -> () + 'static>),
}

pub struct DatasetSubscription {
    pub name: String,
    pub path: String,
}

pub trait Algorithm {
    fn register_events(&self) -> Vec<EventListener>; // Use a lifetime parameter
    fn register_dataset_subscriptions(&self) -> Vec<DatasetSubscription>;
}
