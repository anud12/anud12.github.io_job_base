use std::error::Error;

use log::info;

use super::{request_list::RequestList, request_one::RequestOne};

pub trait FileQuery<Return> {
    fn query_list(&self, query_request: RequestList) -> Result<Vec<Return>, Box<dyn Error>>;
    fn query_one(&self, query_request: RequestOne) -> Result<Return, Box<dyn Error>>;
    fn get_id(&self) -> Option<String>;
    fn find_all(&self) -> Result<Vec<Return>, Box<dyn Error>> {
        let mut request = RequestList::default();
        if let Some(value) = self.get_id() {
            request.parent = Some(value)
        }
        let response = self.query_list(request)?;
        Ok(response)
    }

    fn find_by_name(&self, name: &str) -> Result<Vec<Return>, Box<dyn Error>> {
        let mut request = RequestList::default();
        if let Some(value) = self.get_id() {
            request.parent = Some(value)
        }
        request.name = Some(name.into());
        let response = self.query_list(request)?;

        Ok(response)
    }

    fn find_one_by_name(&self, name: &str) -> Result<Return, Box<dyn Error>> {
        let mut request = RequestOne::default();
        if let Some(value) = self.get_id() {
            request.parent = Some(value)
        }
        request.name = Some(name.into());
        self.query_one(request)
    }
    fn find_one_by_id(&self, id: &str) -> Result<Return, Box<dyn Error>> {
        let mut request = RequestOne::default();
        if let Some(value) = self.get_id() {
            request.parent = Some(value)
        }
        request.id = Some(id.into());
        self.query_one(request)
    }
}
