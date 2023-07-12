import { Response } from 'node-fetch';

export const parseResponse = async (
    response: Response,
    querySkip: number
): Promise<[number, string[]][]> => {
    const responseString = await response.text()
    const data = JSON.parse(responseString);

    const rows = data.values || [];
    const value = rows.map((row: string[], index: number) => {
        const rowIndex = index + querySkip;
        const rowData: string[] = row.map((cell) => cell.toString());
        return [rowIndex, rowData];
    });

    return value;

}