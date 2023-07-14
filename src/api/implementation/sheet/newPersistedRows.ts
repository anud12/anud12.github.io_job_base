import fetch from "node-fetch";
import {GoogleSheet} from "./GoogleSheet";
import {fetchGoogle} from "../fetchGoogle";

export async function newPersistedRows(sheet: GoogleSheet, data: [number, string[]][]): Promise<void> {
  const persistedData = data.map(([rowNumber, row]) => {
    const range = `Sheet1!${rowNumber + 1}:${rowNumber + 1}`;

    return {
      range,
      majorDimension: "ROWS",
      values: [row],
    };
  });

  const body = {
    valueInputOption: "RAW",
    data: persistedData,
    includeValuesInResponse: false,
    responseValueRenderOption: "UNFORMATTED_VALUE",
    responseDateTimeRenderOption: "FORMATTED_STRING",
  };

  const url = `https://sheets.googleapis.com/v4/spreadsheets/${sheet.fileData.id}/values:batchUpdate`;

  await fetchGoogle(url, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${sheet.session.token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });
}
