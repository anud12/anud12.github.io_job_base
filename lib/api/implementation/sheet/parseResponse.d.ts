import { Response } from 'node-fetch';
export declare const parseResponse: (response: Response, querySkip: number) => Promise<[number, string[]][]>;
