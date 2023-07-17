import { Table } from "../../db/Table";
import { TableQuery } from "../../db/TableQuery";
import { TableQueryBuilder } from "../../db/TableQueryBuilder";
import { TableRow } from "../../db/TableRow";
import { GoogleSession } from "../GoogleSession";
import { GoogleDriveFileData } from "../drive/GoogleDriveFileData";
export type ColumnMap = {
    [key: string]: number;
};
export declare class GoogleSheet implements Table<number> {
    session: GoogleSession;
    fileData: GoogleDriveFileData;
    constructor(session: GoogleSession, fileData: GoogleDriveFileData);
    saveAll(data: TableRow<number>[]): Promise<void>;
    findByQuery(query: TableQuery): Promise<TableRow<number>[]>;
    findBy(): TableQueryBuilder<number>;
}
