import { TableQuery } from "./TableQuery";
import { TableQueryBuilder } from "./TableQueryBuilder";
import { TableRow } from "./TableRow";

export type Table<IdType> = {
    saveAll(data: TableRow<IdType>[]): Promise<void>;
    findByQuery(query: TableQuery): Promise<TableRow<IdType>[]>;
    findBy(): TableQueryBuilder<IdType>;
}