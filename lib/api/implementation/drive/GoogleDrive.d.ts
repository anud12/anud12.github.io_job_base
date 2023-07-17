import { FileQuery } from "../../file/FileQuery";
import { GoogleSession } from "../GoogleSession";
import { GoogleDriveFile } from "./GoogleDriveFile";
import { RequestOne } from "../../file/RequestOne.type";
import { RequestList } from "../../file/RequestList.type";
import { GoogleDriveFileUninitialized } from "./GoogleDriveFileUninitialized";
export declare const googleQueryList: (googleSession: GoogleSession, request: RequestList) => Promise<Array<GoogleDriveFile>>;
export declare const googleQueryOne: (googleSession: GoogleSession, request: RequestOne) => Promise<GoogleDriveFile>;
export declare class GoogleDrive extends FileQuery<GoogleDriveFile, GoogleDriveFileUninitialized> {
    protected googleSession: GoogleSession;
    constructor(googleSession: GoogleSession);
    findOneByIdLazy: (id: string) => GoogleDriveFileUninitialized;
}
