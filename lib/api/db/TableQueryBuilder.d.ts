import { Table } from "./Table";
import { TableRow } from "./TableRow";
export declare class TableQueryBuilder<IdType> {
    private tableQuery;
    private table;
    constructor(value: Table<IdType>);
    size(size: number): this;
    clearSize(): this;
    skip(skip: number): this;
    clearSkip(): this;
    query(): Promise<TableRow<IdType>[]>;
}
