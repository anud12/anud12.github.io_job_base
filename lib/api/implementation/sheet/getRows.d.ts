import { GoogleSheet } from "./GoogleSheet";
import { TableQuery } from "../../db/TableQuery";
export declare const getRows: (sheet: GoogleSheet, query: TableQuery) => Promise<[number, string[]][]>;
