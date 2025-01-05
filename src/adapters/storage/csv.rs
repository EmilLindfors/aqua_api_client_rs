use std::path::PathBuf;
use csv::Writer;
use serde::Serialize;
use crate::{domain::ports::StoragePort, error::Error};

pub struct CsvStorage {
    output_dir: PathBuf,
}

impl StoragePort for CsvStorage {
    fn store_data<T: Serialize + Send + Sync>(
        &mut self,
        data: &[T],
        table_name: &str,
    ) -> Result<(), Error> {
        let path = self.output_dir.join(format!("{}.csv", table_name));
        let mut writer = Writer::from_path(path)?;
        for item in data {
            writer.serialize(item)?;
        }
        writer.flush()?;
        Ok(())
    }
}
