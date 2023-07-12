import fetch from "node-fetch";
import {parseResponse} from "./parseResponse";
import {GoogleSheet} from "./GoogleSheet";
import {TableQuery} from "../../db/TableQuery";

export const getRows = async (sheet: GoogleSheet, query: TableQuery): Promise<[number, string[]][]> => {
  const query_skip = query.skip || 1;
  const url = `https://sheets.googleapis.com/v4/spreadsheets/${sheet.fileData.id}/values`;
  const urlWithSize = query.size
    ? `${url}/Sheet1!${query_skip + 1}:${query.size}`
    : url;

  const urlWithDimension = `${urlWithSize}?majorDimension=ROWS`;
  const response = await fetch(urlWithDimension, {
    headers: {
      Authorization: `Bearer ${sheet.session.token}`,
    },
  });

  return parseResponse(response, query_skip);
};
