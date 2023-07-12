import {Table} from "../../db/Table";
import {TableQuery} from "../../db/TableQuery";
import {TableQueryBuilder} from "../../db/TableQueryBuilder";
import {TableRow} from "../../db/TableRow";
import {GoogleSession} from "../GoogleSession";
import {getColumns} from "./getColumns";
import {newRows} from "./newRows";
import {newPersistedRows} from "./newPersistedRows";
import {getRows} from "./getRows";
import {FileData} from "../drive/prepareRequest";

export type ColumnMap = {
  [key: string]: number;
}

export class GoogleSheet implements Table<number> {

  constructor(public session: GoogleSession, public fileData: FileData) {
  }

  async saveAll(data: TableRow<number>[]): Promise<void> {
    console.log(`GoogleSheet.saveAll(data:${JSON.stringify(data)})`);
    const columns = await getColumns(this);
    let columnMap: ColumnMap = columns.reduce((acc: ColumnMap, value: string, index: number) => {
      acc[value] = index;
      return acc;
    }, {});

    const cleanRows = data.reduce((acc: string[][], row: TableRow<number>) => {
      if (!row.id) {
        const [row_list, new_column_map] = transform(row, columnMap);
        columnMap = new_column_map;
        acc.push(row_list);
      }
      return acc;
    }, []);

    const persistedRows = data.reduce((acc: [number, string[]][], row: TableRow<number>) => {
      if (row.id) {
        const [row_list, new_column_map] = transform(row, columnMap);
        columnMap = new_column_map;
        acc.push([row.id, row_list]);
      }
      return acc;
    }, []);

    const columnRow = Object.entries(columnMap).reduce<string[]>(
      (acc, [key, value]: [string, number]) => {
        acc[value] = key;
        return acc;
      },
      Array(Object.keys(columnMap).length).fill('')
    )

    persistedRows.push([0, columnRow]);

    await newPersistedRows(this, persistedRows);
    await newRows(this, cleanRows);
  }

  async findByQuery(query: TableQuery): Promise<TableRow<number>[]> {
    console.log(`GoogleSheet.findByQuery(query:${JSON.stringify(query)})`);
    const columns = await getColumns(this);
    const data = await getRows(this, query);

    return data.map(([id, row]: [number, string[]]) => {
      const map = row.reduce((acc: { [key: string]: string }, value, index) => {
        const key = columns[index];
        if (value) {
          acc[key] = value;
        }
        return acc;
      }, {});
      return {
        id: id,
        data: map
      } satisfies TableRow<number>;
    });
  }

  findBy(): TableQueryBuilder<number> {
    return new TableQueryBuilder(this);
  }
}

function transform(row: TableRow<number>, column_map: ColumnMap): [string[], ColumnMap] {
  const existing_rows_list: string[] = Array(column_map.length).fill('');

  const data = {...row.data};
  for (const [key, value] of Object.entries(data)) {
    if (column_map[key] === undefined) {
      column_map[key] = Object.keys(column_map).length;
      existing_rows_list.push(value);
    } else {
      existing_rows_list[column_map[key]] = value;
      delete data[key];
    }
  }

  return [existing_rows_list, column_map];
}