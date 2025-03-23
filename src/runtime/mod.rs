use std::cell::RefCell;
use std::rc::Rc;

use crate::broker::{AlgorithmAccess, Broker};

pub struct Runtime {
    pub broker: Rc<RefCell<dyn AlgorithmAccess>>,
}

impl Runtime {
    pub fn new(broker: Rc<RefCell<dyn AlgorithmAccess>>) -> Self {
        Self { broker }
    }
}
