use std::{collections::HashMap, error::Error};

use crate::{
    api::db::{IntoTable, Table, TableQuery, TableRow},
    implementation::drive::google_drive_file::GoogleDriveFile,
    FileMetadata,
};

use super::{get, save, Sheet};

impl IntoTable<Sheet> for GoogleDriveFile {
    fn into_table(&self) -> Sheet {
        Sheet {
            session: self.get_session(),
            spreadsheet_id: self.get_id(),
        }
    }
}
fn transform(
    row: &TableRow<u64>,
    column_map: &HashMap<String, usize>,
) -> (Vec<String>, HashMap<String, usize>) {
    let mut column_map = column_map.clone();
    let mut data = row.get_data().clone();
    let mut existing_rows_list = column_map.iter().fold(
        vec![String::new(); column_map.len()],
        |mut acc, (key, value)| {
            acc[value.clone()] = match data.remove(key) {
                Some(value) => value,
                None => "".into(),
            };
            acc
        },
    );
    row.get_data().iter().for_each(|(key, value)| {
        if !column_map.contains_key(&key.to_string()) {
            column_map.insert(key.to_string(), column_map.len());
            existing_rows_list.push(value.clone());
        }
    });
    return (existing_rows_list, column_map);
}
impl Table for Sheet {
    type IdType = u64;
    fn save_all(&self, data: Vec<TableRow<Self::IdType>>) -> Result<(), Box<dyn Error>> {
        let columns = get::columns(self)?;

        let mut column_map = columns.iter().enumerate().fold(
            HashMap::<String, usize>::new(),
            |mut acc, (index, value)| {
                acc.insert(value.clone(), index);
                acc
            },
        );
        let clean_rows = data.iter().filter(|row| row.get_id() == &None).fold(
            Vec::<Vec<String>>::with_capacity(data.len()),
            |mut acc, row| {
                let (row_list, new_column_map) = transform(row, &column_map);
                column_map = new_column_map;
                acc.push(row_list);
                acc
            },
        );
        let mut persisted_rows = data.iter().filter(|row| row.get_id() != &None).fold(
            Vec::<(Self::IdType, Vec<String>)>::with_capacity(data.len()),
            |mut acc, row| {
                let (row_list, new_column_map) = transform(row, &column_map);
                column_map = new_column_map;
                acc.push((row.get_id().unwrap(), row_list));
                acc
            },
        );
        let column_row = column_map.iter().fold(
            vec![String::new(); column_map.len()],
            |mut acc, (key, value)| {
                acc[value.clone()] = key.clone();
                acc
            },
        );
        persisted_rows.push((0, column_row));
        save::persisted_rows(self, persisted_rows)?;
        save::new_rows(self, clean_rows)?;
        Ok(())
    }

    fn find_by_query(
        &self,
        query: TableQuery,
    ) -> Result<Vec<TableRow<Self::IdType>>, Box<dyn Error>> {
        let columns = get::columns(&self)?;
        let data = get::rows(&self, query)?;
        let data = data
            .iter()
            .map(|(id, row)| {
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
                TableRow::new_persisted(id.clone(), map)
            })
            .collect();

        Ok(data)
    }
}
