mod api;
mod implementation;
mod printable;

pub use crate::implementation::google_session::GoogleSession as GoogleSession;
pub use crate::api::file::FileMetadata as FileMetadata;
pub use crate::api::file::FolderQuery as FolderQuery;
pub use crate::api::file::RootQuery as RootQuery;
pub use crate::printable::Printable as Printable;
pub use crate::printable::PrintableAnd as PrintableAnd;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::api::file::{FileMetadata, FolderQuery, RootQuery};
    use crate::GoogleSession;
    use crate::printable::Printable;

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
        let _boxes_trash = global_fs.find_one_by_name("boxes_trash")?;
        let first = boxes.find_by_name("first.json")?.remove(0);

        first.get_body_json().print();
        global_fs.find_all()?.print();
        Ok(())
    }
}
