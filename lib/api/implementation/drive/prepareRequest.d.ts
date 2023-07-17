import { RequestList } from "../../file/RequestList.type";
import { GoogleDriveFileData } from "./GoogleDriveFileData";
export declare const prepareRequest: (token: string, requestList: RequestList) => Promise<Array<GoogleDriveFileData>>;
