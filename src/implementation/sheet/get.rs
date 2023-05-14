use std::error::Error;

use crate::api::db::TableQuery;

use super::{parse_response::parse_response, Sheet};

pub fn rows(sheet: &Sheet, query: TableQuery) -> Result<Vec<(u64, Vec<String>)>, Box<dyn Error>> {
    let query_skip = query.skip.or(0.into()).unwrap();
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values",
        sheet.spreadsheet_id
    );
    let url = match query.size {
        Some(size) => {
            let skip = query_skip;
            let skip = skip + 1;
            format!("{}/Sheet1!{}:{}", url, skip, size)
        }
        None => url,
    };

    let url = format!("{}{}", url, "?majorDimension=ROWS");
    let response = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", sheet.session.token))
        .call()?;

    Ok(parse_response(response, query_skip)?)
}

pub fn columns(sheet: &Sheet) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/Sheet1!1:1",
        sheet.spreadsheet_id
    );
    let url = format!("{}{}", url, "?majorDimension=ROWS");
    let response = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", sheet.session.token))
        .call()?;
    let mut body = parse_response(response, 0)?;
    if body.len() == 0 {
        return Ok(Vec::new());
    }
    let (_, column_list) = body.remove(0);
    Ok(column_list)
}
