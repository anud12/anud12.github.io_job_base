import fetch from "node-fetch";
import {GoogleSheet} from "./GoogleSheet";
import {fetchGoogle} from "../fetchGoogle";

export async function newRows(sheet: GoogleSheet, data: string[][]): Promise<void> {
  if (data.length > 0) {
    const body = {
      range: "Sheet1",
      majorDimension: "ROWS",
      values: data,
    };

    const url = `https://sheets.googleapis.com/v4/spreadsheets/${sheet.fileData.id}/values/Sheet1:append?valueInputOption=RAW&insertDataOption=INSERT_ROWS&includeValuesInResponse=false&responseValueRenderOption=UNFORMATTED_VALUE&responseDateTimeRenderOption=FORMATTED_STRING`;

    await fetchGoogle(url, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${sheet.session.token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });
  }
}