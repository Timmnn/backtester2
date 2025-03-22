use std::{collections::HashMap, path::PathBuf};

use chrono::NaiveDateTime;

pub struct Dataset {
    tickers: HashMap<String, Ticker>,
    name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Quote {
    close: u32,
    time: NaiveDateTime,
}

pub struct Ticker {
    name: String,
    data_path: PathBuf,
    data: Vec<Quote>,
    data_pointer: usize,
}

pub struct DataService {
    datasets: HashMap<String, Dataset>,
}

impl DataService {
    pub fn new() -> Self {
        Self {
            datasets: HashMap::new(),
        }
    }

    pub fn add_dataset(&mut self, dataset: Dataset) {
        self.datasets.insert(dataset.name.clone(), dataset);
    }

    pub fn get_next_data_times_for_all_datasets(&self) -> HashMap<String, Quote> {
        let mut times = HashMap::new();

        for (dataset_name, dataset) in self.datasets.iter() {
            for (ticker_name, ticker) in &dataset.tickers {
                times.insert(
                    ticker_name.clone(),
                    ticker.data[ticker.data_pointer].clone(),
                );
            }
        }

        return times;
    }
}
