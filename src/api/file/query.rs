use std::error::Error;

use super::{folder_query::FolderQuery, request::Request};

pub trait RootQuery<ChildQuery>
where
    ChildQuery: FolderQuery<ChildQuery>,
{
    fn query(&self, query_request: Request) -> Result<Vec<ChildQuery>, Box<dyn Error>>;
    fn find_all(&self) -> Result<Vec<ChildQuery>, Box<dyn Error>> {
        let request = Request::default();
        let response = self.query(request)?;

        Ok(response)
    }

    fn find_by_name(&self, name: &str) -> Result<Vec<ChildQuery>, Box<dyn Error>> {
        let mut request = Request::default();
        request.name = Some(name.into());

        let response = self.query(request)?;

        Ok(response)
    }

    fn find_one_by_name(&self, name: &str) -> Result<ChildQuery, Box<dyn Error>> {
        let mut request = Request::default();
        request.name = Some(name.into());
        request.size = Some(1);
        request.fixed = Some(true);

        let mut response = self.query(request)?;
        match response.len() {
            1 => Ok(response.swap_remove(0)),
            _ => Err(format!("Invalid retrun in find_one_by_name for '{}'", name).into()),
        }
    }
}
