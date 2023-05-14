use std::error::Error;

use super::{Table, TableQuery, TableRow};

pub struct TableQueryBuilder<'a, IdType> {
    table_query: TableQuery,
    table: &'a dyn Table<IdType = IdType>,
}
impl<'a, IdType> TableQueryBuilder<'a, IdType> {
    #[allow(dead_code)]
    pub fn new(value: &'a dyn Table<IdType = IdType>) -> Self {
        TableQueryBuilder {
            table_query: TableQuery::default(),
            table: value,
        }
    }
    #[allow(dead_code)]
    pub fn size(mut self, size: u64) -> Self {
        self.table_query.size = Some(size);
        self
    }
    #[allow(dead_code)]
    pub fn clear_size(mut self) -> Self {
        self.table_query.size = None;
        self
    }
    #[allow(dead_code)]
    pub fn skip(mut self, skip: u64) -> Self {
        self.table_query.skip = Some(skip);
        self
    }
    #[allow(dead_code)]
    pub fn clear_skip(mut self, skip: u64) -> Self {
        self.table_query.size = Some(skip);
        self
    }
    #[allow(dead_code)]
    pub fn query(self) -> Result<Vec<TableRow<IdType>>, Box<dyn Error>> {
        self.table.find_by_query(self.into())
    }
}
impl<'a, IdType> Into<TableQuery> for TableQueryBuilder<'a, IdType> {
    fn into(self) -> TableQuery {
        self.table_query
    }
}
