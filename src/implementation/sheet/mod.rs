mod google_sheet_request;

use std::fmt::format;
use std::ops::Index;
use std::{error::Error, str::FromStr};

use rsa::rand_core::le;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

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
        let mut data = data.clone();
        let data: Vec<serde_json::Value> = data
            .iter()
            .map(|e| {
                let mut row = e.clone();
                let row_number = row.remove(0).replace("\"", "");
                json!({
                    "range": format!("Sheet1!{}:{}", row_number, row_number),
                    "majorDimension": "ROWS",
                    "values": [row],
                })
            })
            .collect();

        let body = json!({
            "valueInputOption": "RAW",
            "data":data,
            "includeValuesInResponse": false,
            "responseValueRenderOption": "UNFORMATTED_VALUE",
            "responseDateTimeRenderOption": "FORMATTED_STRING",
        });

        body.print_pre("Body");

        let url = "https://sheets.googleapis.com/v4/spreadsheets";
        let url = format!("{}/{}", url, self.spreadsheet_id);
        let url = format!("{}/values:batchUpdate", url);
        let request = ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", self.session.token))
            .send_json(body);
        request.print_pre("Result");
        Ok(())
    }
}
