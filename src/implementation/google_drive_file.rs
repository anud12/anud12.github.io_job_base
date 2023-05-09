use std::error::Error;

use crate::api::file::{FileMetadata, FolderQuery};

use super::{google_session::GoogleSession, prepare_request::prepare_request};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FileDataMimeType {
    JSON,
    Folder,
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
    pub permissions: serde_json::Value,
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

    fn move_to(&self, file_metadata: &dyn FileMetadata) -> Result<(), Box<dyn Error>> {
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

        Ok(())
    }

    fn get_body_json(&self) -> Result<serde_json::Value, Box<dyn Error>> {
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

    fn get_body_string(&self) -> Result<String, Box<dyn Error>> {
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
}
impl FolderQuery<GoogleDriveFile> for GoogleDriveFile {
    fn query(
        &self,
        query_request: crate::api::file::Request,
    ) -> Result<Vec<GoogleDriveFile>, Box<dyn Error>> {
        let file_list = prepare_request(self.session.token.clone(), query_request)?;
        let file_list = file_list
            .iter()
            .map(|file| GoogleDriveFile::new(self.session.clone(), file.clone()))
            .collect();
        Ok(file_list)
    }
}

impl GoogleDriveFile {
    pub fn new(session: GoogleSession, file_data: FileData) -> GoogleDriveFile {
        GoogleDriveFile { session, file_data }
    }
}
