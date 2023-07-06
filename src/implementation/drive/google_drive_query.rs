use std::error::Error;

use crate::GoogleSession;

use super::{google_drive_file::GoogleDriveFile, google_drive_request::prepare_request};

pub fn google_drive_query(
    session: &GoogleSession,
    query_request: crate::api::file::RequestList,
) -> Result<Vec<GoogleDriveFile>, Box<dyn Error>> {
    let file_list = prepare_request(session.token.clone(), query_request)?;
    let file_list = file_list
        .iter()
        .map(|file| GoogleDriveFile::new(session.clone(), file.clone()))
        .collect();
    Ok(file_list)
}
