use std::{error::Error, fs::File};

use crate::{implementation::drive::google_drive_file::GoogleDriveFile, FileQuery};

use super::{metadata::FileMetadata, request::Request};

pub trait FolderQuery<ChildQuery>: FileMetadata
where
    ChildQuery: FolderQuery<ChildQuery> + FileMetadata,
{
    fn get_query(&self) -> Box<dyn FileQuery<ChildQuery>>;
    fn find_all(&self) -> Result<Vec<ChildQuery>, Box<dyn Error>> {
        self.get_query().find_all()
    }

    fn find_by_name(&self, name: &str) -> Result<Vec<ChildQuery>, Box<dyn Error>> {
        self.get_query().find_by_name(name)
    }

    fn find_one_by_name(&self, name: &str) -> Result<ChildQuery, Box<dyn Error>> {
        self.get_query().find_one_by_name(name)
    }
}
