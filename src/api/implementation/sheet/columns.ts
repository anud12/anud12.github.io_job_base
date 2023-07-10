import fetch from "node-fetch";
import { Sheet } from "./sheet";
import { parseResponse } from "./parseResponse";

export const getColumns = async (sheet: Sheet): Promise<string[]> => {
    const url = `https://sheets.googleapis.com/v4/spreadsheets/${sheet.spreadsheet_id}/values/Sheet1!1:1?majorDimension=ROWS`;
    const response = await fetch(url, {
        headers: {
            Authorization: `Bearer ${sheet.session.token}`,
        },
    });

    const body = await parseResponse(response, 0);
    if (body.length === 0) {
        return [];
    }

    const [, column_list] = body.shift() || [];
    if (!column_list) {
        return []
    }
    return column_list;
};