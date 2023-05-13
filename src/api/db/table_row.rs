use std::collections::HashMap;
type Data = HashMap<String, String>;
#[derive(Debug, Clone)]
pub struct TableRow<IdType> {
    id: Option<IdType>,
    data: Data,
}
impl<IdType> TableRow<IdType> {
    #[allow(dead_code)]
    pub fn new_persisted(id: IdType, data: Data) -> TableRow<IdType> {
        TableRow { id: Some(id), data }
    }
    #[allow(dead_code)]
    pub fn new() -> TableRow<IdType> {
        TableRow::new_from(HashMap::default())
    }
    #[allow(dead_code)]
    pub fn new_from(data: Data) -> TableRow<IdType> {
        TableRow {
            id: None,
            data: data,
        }
    }
    pub fn get_data<'a>(&'a self) -> &'a Data {
        &self.data
    }
    pub fn get_id<'a>(&'a self) -> &'a Option<IdType> {
        &self.id
    }
    #[allow(dead_code)]
    pub fn insert<Value: Into<String>>(mut self, k: &str, v: Value) -> Self {
        self.data.insert(k.into(), v.into());
        self
    }
    #[allow(dead_code)]
    pub fn get(&self, k: &str) -> Option<String> {
        match self.data.get(&k.to_string()) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
}
