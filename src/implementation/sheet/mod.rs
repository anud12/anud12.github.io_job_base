mod google_sheet_request;

use std::ops::Index;
use std::{error::Error, str::FromStr};

use rsa::rand_core::le;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    api::db::{IntoTable, Table, TableQuery},
    FileMetadata, GoogleSession, Printable, PrintableAnd,
};

use self::google_sheet_request::prepare_request;

use super::drive::google_drive_file::GoogleDriveFile;

pub struct Sheet {
    session: GoogleSession,
    spreadsheet_id: String,
}

impl Sheet {
    pub fn new(session: GoogleSession, spreadsheet_id: String) -> Sheet {
        Sheet {
            session,
            spreadsheet_id,
        }
    }
}
impl IntoTable<Sheet> for GoogleDriveFile {
    fn into_table(&self) -> Sheet {
        Sheet {
            session: self.get_session(),
            spreadsheet_id: self.get_id(),
        }
    }
}
impl Table for Sheet {
    fn query(&self, query: TableQuery) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        prepare_request(
            self.session.token.clone(),
            self.spreadsheet_id.clone(),
            query,
            false,
        )
    }
    fn get_columns(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut query = TableQuery::default();
        query.size = Some(1);
        let mut result = prepare_request(
            self.session.token.clone(),
            self.spreadsheet_id.clone(),
            query,
            true,
        )?;
        match result.len() {
            0 => Err("No header present".into()),
            _ => {
                let mut row = result.remove(0);
                row.remove(0);
                row.insert(0, "row_number".to_string());
                Ok(row)
            }
        }
    }

    fn persist(&self, data: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
        data.print_pre("Persist");
        Ok(())
    }
}
