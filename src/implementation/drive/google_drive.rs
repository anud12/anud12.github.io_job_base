use log::info;

use crate::{api::file::RequestOne, FileQuery, GoogleSession};
use std::error::Error;

use super::{
    google_drive_file::GoogleDriveFile, google_drive_query::google_drive_query,
    google_drive_query_one::google_drive_query_one,
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
        info!("query_list {:?}", query_request);
        google_drive_query(&self.session, query_request)
    }

    fn query_one(&self, query_request: RequestOne) -> Result<GoogleDriveFile, Box<dyn Error>> {
        info!("query_one {:?}", query_request);
        google_drive_query_one(&self.session, query_request)
    }

    fn get_id(&self) -> Option<String> {
        None
    }
}
