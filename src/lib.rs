mod api;
mod implementation;
mod printable;

pub type GoogleSession = implementation::GoogleSession;
pub type FileMetadata = dyn api::file::FileMetadata;
pub type FolderQuery<ChildQuery> = dyn api::file::FolderQuery<ChildQuery>;
pub type RootQuery<ChildQuery> = dyn api::file::RootQuery<ChildQuery>;
pub type Printable = dyn printable::Printable;
pub type PrintableAnd = dyn printable::PrintableAnd;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::api::file::{FileMetadata, FolderQuery, RootQuery};
    use crate::implementation::GoogleSession;
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        // Load the service account JSON file
        // let sa_file = include_str!("service_account.json");
        // let sa_json: serde_json::Value = serde_json::from_str(&sa_file).unwrap();
        // let private_key = sa_json["private_key"].as_str().unwrap();
        // let client_email = sa_json["client_email"].as_str().unwrap();
        let private_key = "";
        let client_email = "";
        let global_fs = GoogleSession::new(client_email, private_key)?.drive();
        let boxes = global_fs.find_one_by_name("boxes")?;
        let boxes_trash = global_fs.find_one_by_name("boxes_trash")?;
        let first = boxes.find_by_name("first.json")?.remove(0);

        first.get_body_json().print();
        global_fs.find_all()?.print();
        Ok(())
    }
}
