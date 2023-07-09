use crate::{api::file::FileMetadata, FileQuery, GoogleSession};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

use super::{
    google_drive_query::google_drive_query, google_drive_query_one::google_drive_query_one,
};

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
        _ => Ok(FileDataMimeType::Unknown(s)),
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
    type File = GoogleDriveFile;
    fn get_id(&self) -> String {
        self.file_data.id.clone()
    }

    fn get_name(&self) -> String {
        self.file_data.name.clone()
    }

    fn get_link(&self) -> String {
        format!(
            "https://drive.google.com/uc?id={}",
            FileMetadata::get_id(self)
        )
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

    fn create<Body: Into<Vec<u8>>>(
        &self,
        name: &str,
        content_type: &str,
        body: Body,
    ) -> Result<GoogleDriveFile, Box<dyn Error>> {
        let form_data: Vec<u8> = body.into();

        let id_token = self.session.clone().token;

        let resumable_url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable";
        let resumable_req: ureq::Response = ureq::post(resumable_url)
            .set("Authorization", &format!("Bearer {}", id_token))
            .set("Content-Type", "application/json; charset=UTF-8")
            .send_json(serde_json::json!({
                "name": name,
                "parents": [self.file_data.id],
                "mimeType": content_type,
            }))?;

        let location = resumable_req.header("Location").unwrap().to_string();

        let put_req = ureq::put(&location)
            .set("Authorization", &format!("Bearer {}", id_token))
            .set("Content-Type", content_type)
            .set("Content-Length", &form_data.len().to_string())
            .send_bytes(&form_data)?;

        let json: serde_json::Value = put_req.into_json()?;
        self.find_one_by_id(json.get("id").map(|f| f.as_str()).unwrap().unwrap())
    }
}
impl FileQuery<GoogleDriveFile> for GoogleDriveFile {
    fn query_list(
        &self,
        query_request: crate::api::file::RequestList,
    ) -> Result<Vec<GoogleDriveFile>, Box<dyn Error>> {
        google_drive_query(&self.session, query_request)
    }

    fn query_one(
        &self,
        query_request: crate::api::file::RequestOne,
    ) -> Result<GoogleDriveFile, Box<dyn Error>> {
        google_drive_query_one(&self.session, query_request)
    }

    fn get_id(&self) -> Option<String> {
        Some(self.file_data.id.clone())
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
