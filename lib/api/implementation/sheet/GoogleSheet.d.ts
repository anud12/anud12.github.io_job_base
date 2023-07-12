import { Table } from "../../db/Table";
import { TableQuery } from "../../db/TableQuery";
import { TableQueryBuilder } from "../../db/TableQueryBuilder";
import { TableRow } from "../../db/TableRow";
import { GoogleSession } from "../GoogleSession";
import { FileData } from "../drive/prepareRequest";
export type ColumnMap = {
    [key: string]: number;
};
export declare class GoogleSheet implements Table<number> {
    session: GoogleSession;
    fileData: FileData;
    constructor(session: GoogleSession, fileData: FileData);
    saveAll(data: TableRow<number>[]): Promise<void>;
    findByQuery(query: TableQuery): Promise<TableRow<number>[]>;
    findBy(): TableQueryBuilder<number>;
}
