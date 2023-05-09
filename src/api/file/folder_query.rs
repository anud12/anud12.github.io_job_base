use std::error::Error;

use super::{metadata::FileMetadata, request::Request};

pub trait FolderQuery<ChildQuery>: FileMetadata
where
    ChildQuery: FolderQuery<ChildQuery> + FileMetadata,
{
    fn query(&self, query_request: Request) -> Result<Vec<ChildQuery>, Box<dyn Error>>;

    fn find_all(&self) -> Result<Vec<ChildQuery>, Box<dyn Error>> {
        let mut request = Request::default();
        request.parent = Some(self.get_id());

        let response = self.query(request)?;
        Ok(response)
    }

    fn find_by_name(&self, name: &str) -> Result<Vec<ChildQuery>, Box<dyn Error>> {
        let mut request = Request::default();
        request.name = Some(name.into());
        request.parent = Some(self.get_id());

        let response = self.query(request)?;

        Ok(response)
    }

    fn find_one_by_name(&self, name: &str) -> Result<ChildQuery, Box<dyn Error>> {
        let mut request = Request::default();
        request.name = Some(name.into());
        request.size = Some(1);
        request.fixed = Some(true);
        request.parent = Some(self.get_id());

        let mut response = self.query(request)?;

        match response.len() {
            1 => Ok(response.remove(0)),
            _ => Err(format!("Invalid retrun in find_one_by_name for '{}'", name).into()),
        }
    }
}
