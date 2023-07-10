import { TableQuery } from "./TableQuery";
import { TableQueryBuilder } from "./TableQueryBuilder";
import { TableRow } from "./TableRow";

export type Table<IdType> = {
    save_all(data: TableRow<IdType>[]): Promise<void>;
    find_by_query(query: TableQuery): Promise<TableRow<IdType>[]>;
    find_by(): TableQueryBuilder<IdType>;
}