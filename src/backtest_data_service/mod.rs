use chrono::NaiveDateTime;
use polars_core::prelude::*;
use polars_io::prelude::*;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::{collections::HashMap, fs::File};

pub struct Dataset {
    tickers: HashMap<String, Ticker>,
    name: String,
}

pub struct DatasetSubscription {
    ticker: String,
    dataset: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Quote {
    close: u32,
    time: NaiveDateTime,
}

pub struct Ticker {
    pub name: String,
    pub data_path: PathBuf,
    pub data: DataFrame,
    pub data_pointer: usize,
}

pub struct DataService {
    datasets: HashMap<String, Dataset>,
    data_directory: PathBuf,
}

impl DataService {
    pub fn new(data_directory: String) -> Self {
        Self {
            datasets: HashMap::new(),
            data_directory: PathBuf::from(data_directory),
        }
    }

    pub fn add_dataset(&mut self, subscription: DatasetSubscription) {
        let files = fs::read_dir(&self.data_directory).unwrap();

        let dataset = Dataset {
            name: subscription.dataset,
            tickers: HashMap::new(),
        };

        for entry in files {
            let file_path = entry.unwrap().path();

            let path_str = file_path.to_str().unwrap();
            let file_name = path_str.split("/").last().unwrap();
            println!("{}", file_name);
        }
    }

    pub fn get_next_data_for_all_datasets(&self) -> HashMap<String, Column> {
        let mut data = HashMap::new();

        for (dataset_name, dataset) in self.datasets.iter() {
            for (ticker_name, ticker) in &dataset.tickers {
                data.insert(
                    ticker_name.clone(),
                    ticker.data[ticker.data_pointer].clone(),
                );
            }
        }

        return data;
    }
}

fn read_parquet(path: &str) -> PolarsResult<DataFrame> {
    let r = File::open(path).unwrap();
    let reader = ParquetReader::new(r);
    reader.finish()
}
