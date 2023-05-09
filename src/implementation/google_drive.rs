use crate::api::file::RootQuery;
use std::error::Error;

use super::{
    google_drive_file::GoogleDriveFile, google_session::GoogleSession,
    prepare_request::prepare_request,
};

#[derive(Debug)]
pub struct GoogleDrive {
    session: GoogleSession,
}
impl GoogleDrive {
    pub fn new(session: GoogleSession) -> GoogleDrive {
        GoogleDrive { session }
    }
}
impl RootQuery<GoogleDriveFile> for GoogleDrive {
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
