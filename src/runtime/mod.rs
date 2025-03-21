use polars_core::prelude::*;
use polars_io::prelude::*;
use std::fs::File;

pub struct Runtime {}

impl Runtime {
    pub fn request_data(&self, file: &str) -> DataFrame {
        let r = File::open(file).unwrap();
        let reader = ParquetReader::new(r);
        reader.finish().unwrap()
    }
}
