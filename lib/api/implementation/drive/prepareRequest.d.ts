import { RequestList } from "../../file/RequestList.type";
export type FileData = {
    id: string;
    name: string;
    mimeType: string;
    parents?: Array<string>;
};
export declare const prepareRequest: (token: string, requestList: RequestList) => Promise<Array<FileData>>;
