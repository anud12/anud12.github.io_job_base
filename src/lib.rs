mod api;
mod implementation;
mod printable;

pub use crate::api::db::IntoTable;
pub use crate::api::db::Table;
pub use crate::api::db::TableRow;
pub use crate::api::file::FileMetadata;
pub use crate::api::file::FileQuery;
pub use crate::implementation::google_session::GoogleSession;
pub use crate::printable::PostPrintable;
pub use crate::printable::PrintableAnd;
pub use serde;
pub use serde_json;

#[cfg(test)]
mod tests_drive {
    use serde_json::json;
    use ureq::Agent;

    use crate::api::file::FileQuery;

    use crate::{FileMetadata, GoogleSession, PostPrintable, PrintableAnd};
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
    #[test]
    fn drive_by_id_works() -> Result<(), Box<dyn Error>> {
        std::env::set_var("PRIVATE_KEY", include_str!("private_key"));
        std::env::set_var("CLIENT_EMAIL", include_str!("client_email"));
        let global_fs = GoogleSession::new()?.into_drive();
        let boxes = global_fs.find_one_by_id("1H1M6KPxCkKsoYR6HaSoQ5jM665ZLq8BQ")?;
        boxes.print("Boxes");
        Ok(())
    }

    #[test]
    fn drive_upload_file() -> Result<(), Box<dyn Error>> {
        std::env::set_var("PRIVATE_KEY", include_str!("private_key"));
        std::env::set_var("CLIENT_EMAIL", include_str!("client_email"));
        let global_fs = GoogleSession::new()?.into_drive();
        let boxes = global_fs.find_one_by_id("1DwTbUSWf5kzNq84Kc08bJ9Wyw9ijfBuS")?;

        boxes
            .create(
                "demo",
                "application/json",
                json!({
                    "name":"value",
                    "time":"1999",
                })
                .to_string(),
            )?
            .print_and("File pre move")
            .get_link()
            .print("Link: ");
        Ok(())
    }

    #[test]
    fn drive_move_from_root() -> Result<(), Box<dyn Error>> {
        std::env::set_var("PRIVATE_KEY", include_str!("private_key"));
        std::env::set_var("CLIENT_EMAIL", include_str!("client_email"));
        let global_fs = GoogleSession::new()?.into_drive();
        let trash = global_fs.find_one_by_id("1d6SljhqtSuaLfIwVbxDGb2AI25PxgR1C")?;
        global_fs
            .find_all_in("0AC35-q6SPyEbUk9PVA")
            .print_and("List")?
            .iter_mut()
            .for_each(|file| {
                file.move_to(&trash).unwrap();
            });

        Ok(())
    }
    #[test]
    fn drive_find_in_root() -> Result<(), Box<dyn Error>> {
        env_logger::init();
        std::env::set_var("PRIVATE_KEY", include_str!("private_key"));
        std::env::set_var("CLIENT_EMAIL", include_str!("client_email"));
        let global_fs = GoogleSession::new()?.into_drive();
        global_fs
            .find_all_in("1YQPVPHBAX7q3sXevT7In2O-VTy9p8BH6")?
            .print("Files: ");
        Ok(())
    }
    #[test]
    fn delete_root() -> Result<(), Box<dyn Error>> {
        std::env::set_var("PRIVATE_KEY", include_str!("private_key"));
        std::env::set_var("CLIENT_EMAIL", include_str!("client_email"));
        let session = GoogleSession::new()?;

        let global_fs = session.into_drive();
        global_fs
            .find_all_in("0AC35-q6SPyEbUk9PVA")
            .print_and("List")?
            .iter_mut()
            .for_each(|file| {
                let url = format!(
                    "https://www.googleapis.com/drive/v3/files/{}",
                    file.get_id()
                );
                let agent = Agent::new();
                agent
                    .delete(&url)
                    .set("Authorization", &format!("Bearer {}", session.token))
                    .call()
                    .unwrap();
            });

        Ok(())
    }
}

mod tests_sheet {
    use crate::{FileQuery, GoogleSession, IntoTable, Table, TableRow};

    #[test]
    fn sheet_works() -> Result<(), Box<dyn std::error::Error>> {
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
