use std::error::Error;

use super::{table_query_builder::TableQueryBuilder, table_row::TableRow, TableQuery};

pub trait IntoTable<TableType: Table> {
    fn into_table(&self) -> TableType;
}

pub trait Table {
    type IdType;
    fn save_all(&self, data: Vec<TableRow<Self::IdType>>) -> Result<(), Box<dyn Error>>;
    fn find_by_query(
        &self,
        query: TableQuery,
    ) -> Result<Vec<TableRow<Self::IdType>>, Box<dyn Error>>;
    fn find_by<'a>(&'a self) -> TableQueryBuilder<'a, Self::IdType>
    where
        Self: Sized,
    {
        TableQueryBuilder::<'a, Self::IdType>::new(self)
    }
}
