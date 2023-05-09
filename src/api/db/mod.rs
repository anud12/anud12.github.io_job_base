use std::{collections::HashMap, error::Error};

use crate::Printable;

#[derive(Default)]
pub struct TableQuery {
    pub size: Option<u64>,
    pub skip: Option<u64>,
}

pub trait Table {
    fn query(&self, query: TableQuery) -> Result<Vec<Vec<String>>, Box<dyn Error>>;
    fn get_columns(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn find(&self, query: TableQuery) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        let columns = self.get_columns()?;
        let data = self.query(query)?;
        data.print();
        let data = data
            .iter()
            .map(|row| {
                row.print_pre("Row data");
                let map = columns.iter().enumerate().fold(
                    HashMap::<String, String>::new(),
                    |mut acc, (index, key)| {
                        let value = row.get(index);
                        if value.is_none() {
                            return acc;
                        }

                        acc.insert(key.to_string(), value.unwrap().to_string());
                        acc
                    },
                );
                map.print_pre("Map data");
                serde_json::to_value(map).expect("serialize map")
            })
            .collect();

        Ok(data)
    }
}
pub trait IntoTable<TableType: Table> {
    fn into_table(&self) -> TableType;
}
