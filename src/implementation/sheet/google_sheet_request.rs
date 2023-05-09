use std::error::Error;

use crate::api::db::TableQuery;

pub fn prepare_request(
    token: String,
    spreadsheet_id: String,
    query: TableQuery,
    get_headers: bool,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let url = match get_headers {
        true => format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/Sheet1!1:1",
            spreadsheet_id
        ),
        false => {
            let url = format!(
                "https://sheets.googleapis.com/v4/spreadsheets/{}/values",
                spreadsheet_id
            );
            let url = match query.size {
                Some(size) => {
                    let skip = query.skip.or(1.into()).unwrap();
                    let skip = skip + 1;
                    format!("{}/Sheet1!{}:{}", url, skip, size)
                }
                None => url,
            };
            url
        }
    };

    let url = format!("{}{}", url, "?majorDimension=ROWS");
    let response = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()?;
    let response = response.into_json::<serde_json::Value>()?;
    let rows = response
        .as_object()
        .expect("response should be object")
        .get("values")
        .expect("response should have range");
    let header = rows.as_array().expect("Rows should be array");

    let value: Vec<Vec<String>> = header
        .iter()
        .map(|row| {
            let row = row.as_array().expect("Row to be array");
            row.iter()
                .map(|cell| cell.as_str().expect("cell to be string").into())
                .collect()
        })
        .collect();
    Ok(value)
}
