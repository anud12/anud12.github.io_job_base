import { Table } from "../../db/Table";
import { TableQuery } from "../../db/TableQuery";
import { TableQueryBuilder } from "../../db/TableQueryBuilder";
import { TableRow } from "../../db/TableRow";
import { GoogleSession } from "../GoogleSession";
import { getColumns } from "./columns";
import { newRows } from "./newRows";
import { persistedRows } from "./persistedRows";
import { getRows } from "./rows";

export class Sheet {
  session: GoogleSession; // Replace 'any' with the appropriate session type
  spreadsheet_id: string;

  constructor(session: any, spreadsheet_id: string) {
    this.session = session;
    this.spreadsheet_id = spreadsheet_id;
  }
}

export type ColumnMap = {
  [key: string]: number;
}

export class SheetTable implements Table<number> {
  idType: number;
  sheet: Sheet;

  constructor(sheet: Sheet) {
    this.idType = 0; // Replace with the appropriate IdType
    this.sheet = sheet;
  }

  async save_all(data: TableRow<number>[]): Promise<void> {
    const columns = await getColumns(this.sheet);
    let column_map: ColumnMap = columns.reduce((acc: ColumnMap, value: string, index: number) => {
      acc[value] = index;
      return acc;
    }, {});

    const clean_rows = data.reduce((acc: string[][], row: TableRow<number>) => {
      if (!row.id) {
        const [row_list, new_column_map] = transform(row, column_map);
        column_map = new_column_map;
        acc.push(row_list);
      }
      return acc;
    }, []);

    const persisted_rows = data.reduce((acc: [number, string[]][], row: TableRow<number>) => {
      if (row.id) {
        const [row_list, new_column_map] = transform(row, column_map);
        column_map = new_column_map;
        acc.push([row.id, row_list]);
      }
      return acc;
    }, []);

    const column_row = Object.entries(column_map).reduce(
      (acc: string[], [key, value]: [string, number]) => {
        acc[value] = key;
        return acc;
      },
      Array(Object.keys(column_map).length).fill('')
    );

    persisted_rows.push([0, column_row]);

    await persistedRows(this.sheet, persisted_rows);
    await newRows(this.sheet, clean_rows);
  }

  async find_by_query(query: TableQuery): Promise<TableRow<number>[]> {
    const columns = await getColumns(this.sheet);
    const data = await getRows(this.sheet, query);

    const parsedData = data.map(([id, row]: [number, string[]]) => {
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

    return parsedData;
  }

  find_by(): TableQueryBuilder<number> {
    return new TableQueryBuilder(this);
  }
}

function transform(row: TableRow<number>, column_map: ColumnMap): [string[], ColumnMap] {
  const existing_rows_list: string[] = Array(column_map.length).fill('');

  const data = { ...row.data };
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