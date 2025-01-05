use postgres::{Client, NoTls};

use crate::{domain::ports::StoragePort, error::Error};

pub struct PostgresStorage {
    client: Client,
}

impl PostgresStorage {
    pub fn new(host: &str, user: &str, password: &str) -> Self {
        let client = Client::connect(&format!("host={host} user={user} password={password}"), NoTls).unwrap();
        PostgresStorage { client }
    }
}

impl StoragePort for PostgresStorage {
    fn store_data<T: serde::Serialize + Send + Sync>(
            &mut self,
            data: &[T],
            table_name: &str,
        ) -> Result<(), Error> {
        let mut query = format!("INSERT INTO {} VALUES ", table_name);
        for item in data {
            query.push_str(&serde_json::to_string(item).map_err(|e| Error::Serde(e.to_string()))?);
            query.push(',');
        }
        query.pop(); // Remove trailing comma
        self.client.execute(&query, &[])?;
        Ok(())
    }
}