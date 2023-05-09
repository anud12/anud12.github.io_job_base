use std::error::Error;

use super::google_drive_file::FileData;

pub fn prepare_request(
    token: String,
    arguments: crate::api::file::Request,
) -> Result<Vec<FileData>, Box<dyn Error>> {
    let mut vec: Vec<String> = vec![];

    if let Some(value) = arguments.name {
        vec.push(format!("name='{}'", value));
    }
    if let Some(value) = arguments.parent {
        vec.push(format!("'{}' in parents", value));
    }
    let query = vec.join(" and ");
    let response = ureq::get("https://www.googleapis.com/drive/v3/files")
        .query("fields", "files(id,name, mimeType, parents, permissions)") // change this to the fields you need
        .query("q", &query)
        .set("Authorization", &format!("Bearer {}", token))
        .call()?;

    let body = response.into_json::<serde_json::Value>()?;
    let files_string = match body.get("files") {
        Some(value) => value.to_string(),
        None => return Err(format!("files value does not exist on body {:?}", body).into()),
    };
    let file_list: Vec<FileData> = serde_json::from_str(&files_string)?;

    Ok(file_list)
}
