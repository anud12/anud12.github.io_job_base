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
        let data = data.clone();

        let persisted_data: Vec<serde_json::Value> = data
            .iter()
            .filter(|e| {
                if e.get(0).is_none() {
                    return false;
                }
                let first = e.get(0);
                return first.unwrap() != "";
            })
            .map(|e| {
                let mut row = e.clone();
                row.print_pre("Row");
                let row_number: u64 = row.remove(0).replace("\"", "").parse().unwrap();
                let row_number = row_number + 1;
                json!({
                    "range": format!("Sheet1!{}:{}", row_number, row_number),
                    "majorDimension": "ROWS",
                    "values": [row],
                })
            })
            .collect();

        if persisted_data.len() > 0 {
            let body = json!({
                "valueInputOption": "RAW",
                "data":persisted_data,
                "includeValuesInResponse": false,
                "responseValueRenderOption": "UNFORMATTED_VALUE",
                "responseDateTimeRenderOption": "FORMATTED_STRING",
            });

            let url = "https://sheets.googleapis.com/v4/spreadsheets";
            let url = format!("{}/{}", url, self.spreadsheet_id);
            let url = format!("{}/values:batchUpdate", url);
            ureq::post(&url)
                .set("Authorization", &format!("Bearer {}", self.session.token))
                .send_json(body)?;
        }

        let new_data: Vec<Vec<String>> = data
            .iter()
            .filter(|e| {
                if e.get(0).is_none() {
                    return true;
                }
                let first = e.get(0);
                return first.unwrap() == "";
            })
            .map(|e| {
                let mut row = e.clone();
                row.remove(0);
                row
            })
            .collect();
        if (new_data.len() > 0) {
            let body = json!({
                // "range": "Sheet1",
                "majorDimension": "ROWS",
                "values": new_data,
            });
            body.print_pre("Body append");
            let url = "https://sheets.googleapis.com/v4/spreadsheets";
            let url = format!("{}/{}", url, self.spreadsheet_id);
            let url = format!("{}/values/Sheet1:append", url);
            let url = format!("{}?", url);
            let url = format!("{}valueInputOption=RAW", url);
            let url = format!("{}&insertDataOption=INSERT_ROWS", url);
            let url = format!("{}&includeValuesInResponse=false", url);
            let url = format!("{}&responseValueRenderOption=UNFORMATTED_VALUE", url);
            let url = format!("{}&responseDateTimeRenderOption=FORMATTED_STRING", url);
            ureq::post(&url)
                .set("Authorization", &format!("Bearer {}", self.session.token))
                .send_json(body)?;
        }

        Ok(())
    }
}
