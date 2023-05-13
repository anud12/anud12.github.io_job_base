mod api;
mod implementation;
mod printable;

pub use crate::api::file::FileMetadata;
pub use crate::api::file::FolderQuery;
pub use crate::api::file::RootQuery;
pub use crate::implementation::google_session::GoogleSession;
pub use crate::printable::PostPrintable;
pub use crate::printable::PrintableAnd;

#[cfg(test)]
mod tests_drive {
    use crate::api::file::{FileMetadata, FolderQuery, RootQuery};
    use crate::printable::PostPrintable;
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

        first.into_json().print_post("a");
        global_fs.find_all()?.print_post("b");
        Ok(())
    }
}

mod tests_sheet {
    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        use crate::{
            api::db::{IntoTable, Table, TableQuery, TableRow},
            GoogleSession, PostPrintable, RootQuery,
        };
        let sa_file = include_str!("service_account.json");
        let sa_json: serde_json::Value = serde_json::from_str(&sa_file).unwrap();
        let private_key = sa_json["private_key"].as_str().unwrap();
        let client_email = sa_json["client_email"].as_str().unwrap();

        let global_fs = GoogleSession::new(client_email, private_key)?.into_drive();
        let db = global_fs.find_one_by_name("demo.db")?.into_table();
        let mut query = TableQuery::default();
        query.size = 100.into();
        let data = db.find(query)?;
        data.print_post("data");
        let mut data: Vec<TableRow<u64>> = data
            .iter()
            .map(|row| {
                let key = row.get("blakey").unwrap();
                row.clone()
                    .insert("blakey".into(), format!("{}{}", key, " value"))
            })
            .collect();
        data.push(
            TableRow::new()
                .insert("nume", "Ionel")
                .insert("prenume", "Popescu")
                .insert("addresa", "312")
                .insert("oras", "Tulcea"),
        );
        db.save_all(data)?;
        Ok(())
    }
}
