use std::error::Error;

pub trait FileMetadata {
    type File;
    fn create<Body: Into<String>>(
        &self,
        name: &str,
        content_type: &str,
        body: Body,
    ) -> Result<Self::File, Box<dyn Error>>;
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_link(&self) -> String;
    fn into_json(&self) -> Result<serde_json::Value, Box<dyn Error>>;
    fn into_string(&self) -> Result<String, Box<dyn Error>>;
    fn move_to<File: FileMetadata>(&mut self, file_metadata: &File) -> Result<(), Box<dyn Error>>;
    fn rename<Name: Into<String>>(&mut self, name: Name) -> Result<(), Box<dyn Error>>;
}
