use std::error::Error;

use serde_json::json;
use ureq::Response;

pub fn parse_response(
    response: Response,
    query_skip: u64,
) -> Result<Vec<(u64, Vec<String>)>, Box<dyn Error>> {
    let response = response.into_json::<serde_json::Value>()?;
    let rows = response
        .as_object()
        .expect("response should be object")
        .get("values");
    let rows = match rows {
        Some(e) => e.clone(),
        None => json!([]),
    };
    let header = rows.as_array().expect("Rows should be array");

    let value = header
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let row = row.as_array().expect("Row to be array");
            let row: Vec<String> = row
                .iter()
                .map(|cell| cell.as_str().expect("cell to be string").into())
                .collect();
            let index: u64 = u64::try_from(index).expect("Transfrom index from usize to u64");
            (index + query_skip, row)
        })
        .collect();
    Ok(value)
}
