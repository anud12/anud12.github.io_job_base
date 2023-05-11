mod api;
mod implementation;
mod printable;

pub use crate::api::file::FileMetadata;
pub use crate::api::file::FolderQuery;
pub use crate::api::file::RootQuery;
pub use crate::implementation::google_session::GoogleSession;
pub use crate::printable::Printable;
pub use crate::printable::PrintableAnd;

#[cfg(test)]
mod tests_drive {
    use crate::api::file::{FileMetadata, FolderQuery, RootQuery};
    use crate::printable::Printable;
    use crate::GoogleSession;
    use std::error::Error;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let sa_file = include_str!("service_account.json");
        let sa_json: serde_json::Value = serde_json::from_str(&sa_file).unwrap();
        let private_key = sa_json["private_key"].as_str().unwrap();
        let client_email = sa_json["client_email"].as_str().unwrap();
        let global_fs = GoogleSession::new(client_email, private_key)?.into_drive();
        let boxes = global_fs.find_one_by_name("boxes")?;
        let _boxes_trash = global_fs.find_one_by_name("boxes_trash")?;
        let first = boxes.find_by_name("first.json")?.remove(0);

        first.into_json().print();
        global_fs.find_all()?.print();
        Ok(())
    }
}

mod tests_sheet {
    use std::error::Error;

    use serde_json::json;

    use crate::{
        api::db::{IntoTable, Table, TableQuery},
        FileMetadata, FolderQuery, GoogleSession, Printable, PrintableAnd, RootQuery,
    };

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let sa_file = include_str!("service_account.json");
        let sa_json: serde_json::Value = serde_json::from_str(&sa_file).unwrap();
        let private_key = sa_json["private_key"].as_str().unwrap();
        let client_email = sa_json["client_email"].as_str().unwrap();

        let global_fs = GoogleSession::new(client_email, private_key)?.into_drive();
        let db = global_fs.find_one_by_name("demo.db")?.into_table();
        let mut query = TableQuery::default();
        query.size = 10.into();
        let mut data = db.find(query)?;
        data.print_pre("data");
        data.push(json!({
            "nume":"Ionel",
            "prenume": "Popsescu",
            "addresa": "312",
            "oras": "Tulcea",
            "oras2": "Demo",
            "oras5": "Demo4",
            "blakey": "blavalue",
        }));
        db.save_all(data)?;
        Ok(())
    }
}
