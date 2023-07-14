import fetch from "node-fetch";
import { GoogleSheet} from "./GoogleSheet";
import { parseResponse } from "./parseResponse";
import {fetchGoogle} from "../fetchGoogle";

export const getColumns = async (sheet: GoogleSheet): Promise<string[]> => {
    const url = `https://sheets.googleapis.com/v4/spreadsheets/${sheet.fileData.id}/values/Sheet1!1:1?majorDimension=ROWS`;
    const response = await fetchGoogle(url, {
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