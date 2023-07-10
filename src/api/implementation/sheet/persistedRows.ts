import fetch from "node-fetch";
import { Sheet } from "./sheet";

export async function persistedRows(sheet: Sheet, data: [number, string[]][]): Promise<void> {
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

  const url = `https://sheets.googleapis.com/v4/spreadsheets/${sheet.spreadsheet_id}/values:batchUpdate`;

  await fetch(url, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${sheet.session.token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });
}
