use std::error::Error;

use serde_json::json;

use super::Sheet;

pub fn persisted_rows(sheet: &Sheet, data: Vec<(u64, Vec<String>)>) -> Result<(), Box<dyn Error>> {
    let persisted_data: Vec<serde_json::Value> = data
        .iter()
        .map(|e| {
            let (row_number, row) = e.clone();
            let row_number = row_number + 1;

            json!({
                "range": format!("Sheet1!{}:{}", row_number, row_number),
                "majorDimension": "ROWS",
                "values": [row],
            })
        })
        .collect();
    let body = json!({
        "valueInputOption": "RAW",
        "data":persisted_data,
        "includeValuesInResponse": false,
        "responseValueRenderOption": "UNFORMATTED_VALUE",
        "responseDateTimeRenderOption": "FORMATTED_STRING",
    });
    let url = "https://sheets.googleapis.com/v4/spreadsheets";
    let url = format!("{}/{}", url, sheet.spreadsheet_id);
    let url = format!("{}/values:batchUpdate", url);
    ureq::post(&url)
        .set("Authorization", &format!("Bearer {}", sheet.session.token))
        .send_json(body)?;

    Ok(())
}

pub fn new_rows(sheet: &Sheet, data: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    if data.len() > 0 {
        let body = json!({
            "range": "Sheet1",
            "majorDimension": "ROWS",
            "values": data,
        });
        let url = "https://sheets.googleapis.com/v4/spreadsheets";
        let url = format!("{}/{}", url, sheet.spreadsheet_id);
        let url = format!("{}/values/Sheet1:append", url);
        let url = format!("{}?", url);
        let url = format!("{}valueInputOption=RAW", url);
        let url = format!("{}&insertDataOption=INSERT_ROWS", url);
        let url = format!("{}&includeValuesInResponse=false", url);
        let url = format!("{}&responseValueRenderOption=UNFORMATTED_VALUE", url);
        let url = format!("{}&responseDateTimeRenderOption=FORMATTED_STRING", url);
        ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", sheet.session.token))
            .send_json(body)?;
    }
    Ok(())
}
