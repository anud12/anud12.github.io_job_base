use std::error::Error;

pub trait FileMetadata {
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn move_to(&self, file_metadata: &dyn FileMetadata) -> Result<(), Box<dyn Error>>;
    fn into_json(&self) -> Result<serde_json::Value, Box<dyn Error>>;
    fn into_string(&self) -> Result<String, Box<dyn Error>>;
}