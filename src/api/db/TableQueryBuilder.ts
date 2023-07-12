import { Table } from "./Table";
import { TableQuery } from "./TableQuery";
import { TableRow } from "./TableRow";

export class TableQueryBuilder<IdType> {
    private tableQuery: TableQuery;
    private table: Table<IdType>;

    constructor(value: Table<IdType>) {
        this.tableQuery = {};
        this.table = value;
    }

    size(size: number): this {
        this.tableQuery.size = size;
        return this;
    }

    clearSize(): this {
        this.tableQuery.size = undefined;
        return this;
    }

    skip(skip: number): this {
        this.tableQuery.skip = skip;
        return this;
    }

    clearSkip(): this {
        this.tableQuery.skip = undefined;
        return this;
    }

    async query(): Promise<TableRow<IdType>[]> {
        return this.table.findByQuery(this.tableQuery);
    }
}