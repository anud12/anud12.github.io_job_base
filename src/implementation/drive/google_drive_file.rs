use std::error::Error;

use crate::{
    api::file::{FileMetadata, FolderQuery},
    FileQuery, GoogleSession,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{google_drive::GoogleDrive, google_drive_request::prepare_request};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FileDataMimeType {
    JSON,
    Folder,
    DB,
    Unknown(String),
}

fn deserialize_file_data_mime_type<'de, D>(deserializer: D) -> Result<FileDataMimeType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "application/json" => Ok(FileDataMimeType::JSON),
        "application/vnd.google-apps.folder" => Ok(FileDataMimeType::Folder),
        "application/vnd.google-apps.spreadsheet" => Ok(FileDataMimeType::DB),
        _ => Err(serde::de::Error::custom(format!(
            "unknown mime type: {}",
            s
        ))),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileData {
    pub id: String,
    pub name: String,
    #[serde(
        alias = "mimeType",
        deserialize_with = "deserialize_file_data_mime_type"
    )]
    pub mime_type: FileDataMimeType,
    pub parents: Option<Vec<String>>,
}
#[derive(Debug)]
pub struct GoogleDriveFile {
    session: GoogleSession,
    file_data: FileData,
}
impl FileMetadata for GoogleDriveFile {
    fn get_id(&self) -> String {
        self.file_data.id.clone()
    }

    fn get_name(&self) -> String {
        self.file_data.name.clone()
    }

    fn into_json(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        let response: serde_json::Value = ureq::get(
            format!(
                "https://www.googleapis.com/drive/v3/files/{}?alt=media",
                self.file_data.id,
            )
            .as_str(),
        )
        .set("Authorization", &format!("Bearer {}", self.session.token))
        .call()?
        .into_json::<serde_json::Value>()?;

        Ok(response)
    }

    fn into_string(&self) -> Result<String, Box<dyn Error>> {
        let response: String = ureq::get(
            format!(
                "https://www.googleapis.com/drive/v3/files/{}?alt=media",
                self.file_data.id,
            )
            .as_str(),
        )
        .set("Authorization", &format!("Bearer {}", self.session.token))
        .call()?
        .into_string()?;

        Ok(response)
    }

    fn move_to<File: FileMetadata>(&mut self, file_metadata: &File) -> Result<(), Box<dyn Error>> {
        match &self.file_data.parents {
            None => return Err("No parents found".into()),
            Some(parents) => {
                ureq::patch(
                    format!(
                        "https://www.googleapis.com/upload/drive/v3/files/{}?removeParents={}&addParents={}",
                        self.file_data.id,
                        parents.get(0).unwrap(),
                        file_metadata.get_id(),
                    )
                    .as_str(),
                )
                .set(
                    "Authorization",
                    &format!("Bearer {}", self.session.token),
                )
                .call()?;
            }
        }
        self.file_data.parents = Some(vec![file_metadata.get_id()]);

        Ok(())
    }

    fn rename<Name: Into<String>>(&mut self, name: Name) -> Result<(), Box<dyn Error>> {
        let name: String = name.into();
        ureq::patch(
            format!(
                "https://www.googleapis.com/drive/v3/files/{}",
                self.file_data.id,
            )
            .as_str(),
        )
        .set("Authorization", &format!("Bearer {}", self.session.token))
        .send_json(json!({
            "name": name,
        }))?;
        self.file_data.name = name;
        Ok(())
    }
}
impl FolderQuery<GoogleDriveFile> for GoogleDriveFile {
    fn get_query(&self) -> Box<dyn FileQuery<GoogleDriveFile>> {
        Box::new(GoogleDrive::new(self.session.clone()))
    }
}

impl GoogleDriveFile {
    pub fn new(session: GoogleSession, file_data: FileData) -> GoogleDriveFile {
        GoogleDriveFile { session, file_data }
    }
    pub fn get_session(&self) -> GoogleSession {
        self.session.clone()
    }
}
