use std::{collections::HashMap, error::Error};

use crate::Printable;

#[derive(Default)]
pub struct TableQuery {
    pub size: Option<u64>,
    pub skip: Option<u64>,
}

pub trait IntoTable<TableType: Table> {
    fn into_table(&self) -> TableType;
}

pub trait Table {
    fn query(&self, query: TableQuery) -> Result<Vec<Vec<String>>, Box<dyn Error>>;
    fn get_columns(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn persist(&self, columns: Vec<String>, data: Vec<Vec<String>>) -> Result<(), Box<dyn Error>>;
    fn save_all(&self, data: Vec<serde_json::Value>) -> Result<(), Box<dyn Error>> {
        let columns = self.get_columns()?;
        let mut column_map = columns.iter().enumerate().fold(
            HashMap::<String, usize>::new(),
            |mut acc, (index, value)| {
                acc.insert(value.to_string(), index);
                acc
            },
        );
        let data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                let map = row.as_object().unwrap().iter().fold(
                    HashMap::<usize, String>::new(),
                    |mut acc, (key, value)| {
                        let index = match column_map.get(key) {
                            Some(value) => value,
                            None => {
                                let index = column_map.len();
                                column_map.insert(key.to_string(), index);
                                column_map.get(key).unwrap()
                            }
                        };
                        acc.insert(index.clone(), value.as_str().unwrap().to_string());
                        acc
                    },
                );
                map.iter().fold(
                    vec![String::new(); column_map.len()],
                    |mut acc, (index, value)| {
                        acc[index.clone()] = value.clone();
                        acc
                    },
                )
            })
            .collect();
        let columns: Vec<String> =
            column_map
                .iter()
                .fold(vec![String::new(); column_map.len()], |mut acc, value| {
                    acc[value.1.clone()] = value.0.clone();
                    acc
                });
        self.persist(columns, data)
    }
    fn find(&self, query: TableQuery) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        let columns = self.get_columns()?;
        let data = self.query(query)?;
        let data = data
            .iter()
            .map(|row| {
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
                serde_json::to_value(map).expect("serialize map")
            })
            .collect();

        Ok(data)
    }
}
