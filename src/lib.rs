mod api;
mod implementation;
mod printable;

pub use crate::api::file::FileMetadata;
pub use crate::api::file::FileQuery;
pub use crate::api::file::FolderQuery;
pub use crate::implementation::google_session::GoogleSession;
pub use crate::printable::PostPrintable;
pub use crate::printable::PrintableAnd;

#[cfg(test)]
mod tests_drive {
    use crate::api::file::{FileQuery, FolderQuery};

    use crate::{FileMetadata, GoogleSession};
    use std::error::Error;

    #[test]
    fn drive_works() -> Result<(), Box<dyn Error>> {
        std::env::set_var("PRIVATE_KEY", include_str!("private_key"));
        std::env::set_var("CLIENT_EMAIL", include_str!("client_email"));
        let global_fs = GoogleSession::new()?.into_drive();
        let boxes = global_fs.find_one_by_name("boxes")?;
        let _boxes_trash = global_fs.find_one_by_name("boxes_trash")?;
        let mut first = boxes.find_by_name("Copy of first.json")?.remove(0);
        first.rename("second.json")?;
        Ok(())
    }
}

mod tests_sheet {

    #[test]
    fn sheet_works() -> Result<(), Box<dyn std::error::Error>> {
        use crate::{
            api::db::{IntoTable, Table, TableRow},
            FileQuery, GoogleSession,
        };
        std::env::set_var("PRIVATE_KEY", include_str!("private_key"));
        std::env::set_var("CLIENT_EMAIL", include_str!("client_email"));
        let global_fs = GoogleSession::new()?.into_drive();
        let db = global_fs.find_one_by_name("demo.db")?.into_table();
        let data = db.find_by().size(100).query()?;
        let mut data: Vec<TableRow<u64>> = data
            .iter()
            .map(|row| {
                let key = row.get("blakey").unwrap();
                row.clone().insert("blakey", format!("{}{}", key, " value"))
            })
            .collect();
        let time = std::time::UNIX_EPOCH.elapsed()?;
        data.push(
            TableRow::new()
                .insert("nume", "Ionel")
                .insert("prenume", "Popescu")
                .insert("addresa", "312")
                .insert("oras", "Tulcea")
                .insert(time.as_secs().to_string(), "Timed"),
        );
        db.save_all(data)?;
        Ok(())
    }
}
