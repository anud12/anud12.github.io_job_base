use std::error::Error;

use crate::{
    api::file::{RequestList, RequestOne},
    GoogleSession,
};

use super::{
    google_drive_file::{FileData, GoogleDriveFile},
    google_drive_request::prepare_request,
};

pub fn google_drive_query_one(
    session: &GoogleSession,
    query_request: RequestOne,
) -> Result<GoogleDriveFile, Box<dyn Error>> {
    let file: FileData = match query_request.id {
        Some(value) => {
            let response =
                ureq::get(format!("https://www.googleapis.com/drive/v3/file/{}", value).as_str())
                    .query("fields", "files(id, name, mimeType, parents)") // change this to the fields you need
                    .set("Authorization", &format!("Bearer {}", session.token))
                    .call()?;
            response.into_json()?
        }
        None => {
            let mut request = RequestList::default();
            request.name = query_request.name.clone();
            request.size = Some(1);
            let mut file_list = prepare_request(session.token.clone(), request)?;
            match file_list.len() == 1 {
                true => file_list.remove(0),
                false => return Err("query_one returned list length different than 1".into()),
            }
        }
    };
    if let Some(value) = query_request.name {
        if value != file.name {
            return Err(format!("query_one file not found with name {}", value).into());
        }
    }
    if let Some(value) = query_request.parent {
        if value != file.name {
            return Err(format!("query_one file not found with parent {}", value).into());
        }
    }
    Ok(GoogleDriveFile::new(session.clone(), file.clone()))
}
