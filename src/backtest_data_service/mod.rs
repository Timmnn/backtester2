use chrono::NaiveDateTime;
use polars_core::prelude::*;
use polars_io::prelude::*;
use std::path::PathBuf;
use std::{collections::HashMap, fs::File};

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
    pub name: String,
    pub data_path: PathBuf,
    pub data: DataFrame,
    pub data_pointer: usize,
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

    pub fn add_dataset(&mut self, path: PathBuf) {
        let data = read_parquet(
            "/Users/timmnicolaizik/Dev/AlgotradingPlatform/data/futures/ES/1H_OHLCV/2018-09-21.parquet",
        ).unwrap();

        let mut tickers = HashMap::new();

        tickers.insert(
            "VX".to_string(),
            Ticker {
                name: "VX".to_string(),
                data_path: "".into(),
                data,
                data_pointer: 0,
            },
        );

        self.datasets.insert(
            "NAME".to_string(),
            Dataset {
                tickers,
                name: "VX".to_string(),
            },
        );
    }

    pub fn get_next_data_for_all_datasets(&self) -> HashMap<String, Quote> {
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
