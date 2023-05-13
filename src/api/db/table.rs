use std::error::Error;

use super::{table_row::TableRow, TableQuery};

pub trait IntoTable<TableType: Table> {
    fn into_table(&self) -> TableType;
}

pub trait Table {
    type IdType;
    fn save_all(&self, data: Vec<TableRow<Self::IdType>>) -> Result<(), Box<dyn Error>>;
    fn find(&self, query: TableQuery) -> Result<Vec<TableRow<Self::IdType>>, Box<dyn Error>>;
    /* {
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
    } */
}
