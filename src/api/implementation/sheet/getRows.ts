import fetch from "node-fetch";
import {parseResponse} from "./parseResponse";
import {GoogleSheet} from "./GoogleSheet";
import {TableQuery} from "../../db/TableQuery";
import {fetchGoogle} from "../fetchGoogle";

export const getRows = async (sheet: GoogleSheet, query: TableQuery): Promise<[number, string[]][]> => {
  const query_skip = query.skip || 1;
  const url = `https://sheets.googleapis.com/v4/spreadsheets/${sheet.fileData.id}/values`;
  const urlWithSize = `${url}/Sheet1!${query_skip + 1}:${query.size ?? 40000}`;

  const urlWithDimension = `${urlWithSize}?majorDimension=ROWS`;
  const response = await fetchGoogle(urlWithDimension, {
    headers: {
      Authorization: `Bearer ${sheet.session.token}`,
    },
  });

  return parseResponse(response, query_skip);
};
