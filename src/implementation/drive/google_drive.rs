use crate::{api::file::RequestOne, FileQuery, GoogleSession};
use std::error::Error;

use super::{
    google_drive_file::GoogleDriveFile, google_drive_query::query,
    google_drive_request::prepare_request,
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
impl FileQuery<GoogleDriveFile> for GoogleDrive {
    fn query_list(
        &self,
        query_request: crate::api::file::RequestList,
    ) -> Result<Vec<GoogleDriveFile>, Box<dyn Error>> {
        query(&self.session, query_request)
    }

    fn query_one(&self, query_request: RequestOne) -> Result<GoogleDriveFile, Box<dyn Error>> {
        let file_list = prepare_request(self.session.token.clone(), query_request)?;
        let file_list = file_list
            .iter()
            .map(|file| GoogleDriveFile::new(self.session.clone(), file.clone()))
            .collect();
        Ok(file_list)
    }
}
