import { Table } from "../../db/Table";
import { TableQuery } from "../../db/TableQuery";
import { TableQueryBuilder } from "../../db/TableQueryBuilder";
import { TableRow } from "../../db/TableRow";
import { GoogleSession } from "../GoogleSession";
import { getColumns } from "./getColumns";
import { newRows } from "./newRows";
import { newPersistedRows } from "./newPersistedRows";
import { getRows } from "./getRows";
import {GoogleDriveFileData} from "../drive/GoogleDriveFileData";

export type ColumnMap = {
  [key: string]: number;
}

export class GoogleSheet implements Table<number> {

  constructor(public session: GoogleSession, public fileData: GoogleDriveFileData) {
  }

  async saveAll(data: TableRow<number>[]): Promise<void> {
    console.log(`GoogleSheet[${this.fileData.id}].saveAll(data:${JSON.stringify(data)})`);
    // Retrieve the columns from the Google Sheet
    const columns = await getColumns(this);

    // Create a mapping of column names to their index
    let columnMap: ColumnMap = columns.reduce((acc: ColumnMap, value: string, index: number) => {
      acc[value] = index;
      return acc;
    }, {});

    // Extract new rows that do not have an ID assigned
    const cleanRows = data.reduce((acc: string[][], row: TableRow<number>) => {
      if (!row.id) {
        const [row_list, new_column_map] = transform(row, columnMap);
        columnMap = new_column_map;
        acc.push(row_list);
      }
      return acc;
    }, []);

    // Extract persisted rows that have an ID assigned
    const persistedRows = data.reduce((acc: [number, string[]][], row: TableRow<number>) => {
      if (row.id) {
        const [row_list, new_column_map] = transform(row, columnMap);
        columnMap = new_column_map;
        acc.push([row.id, row_list]);
      }
      return acc;
    }, []);

    // Create a row for the column names excluding undefined values
    const columnRow = Object.entries(columnMap).reduce<string[]>(
      (acc, [key, value]: [string, number]) => {
        acc[value] = key;
        return acc;
      },
      Array(Object.keys(columnMap).length).fill('')
    );

    // Add the column row to the persisted rows
    persistedRows.push([0, columnRow]);

    // Call functions to persist the new rows and update the existing rows in the Google Sheet
    await newPersistedRows(this, persistedRows);
    await newRows(this, cleanRows);
  }

  async findByQuery(query: TableQuery): Promise<TableRow<number>[]> {
    console.log(`GoogleSheet[${this.fileData.id}].findByQuery(query:${JSON.stringify(query)})`);
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

  const data = { ...row.data };
  for (const [key, value] of Object.entries(data)) {
    if(value === undefined) {
      continue;
    }
    if (column_map[key] === undefined) {
      column_map[key] = Object.keys(column_map).length;
      existing_rows_list.push(value as unknown as string);
    } else {
      existing_rows_list[column_map[key]] = value as unknown as string;
      delete data[key];
    }
  }

  return [existing_rows_list, column_map];
}