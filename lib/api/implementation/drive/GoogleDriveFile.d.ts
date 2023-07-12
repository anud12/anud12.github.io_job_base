import { FileQuery } from "../../file/FileQuery";
import { RequestList } from "../../file/RequestList.type";
import { RequestOne } from "../../file/RequestOne.type";
import { FileMetadata } from "../../file/FileMetadata";
import { GoogleSession } from "../GoogleSession";
import { FileData } from "./prepareRequest";
import { GoogleSheet } from "../sheet/GoogleSheet";
export declare const googleQueryList: (googleSession: GoogleSession, request: RequestList) => Promise<Array<GoogleDriveFile>>;
export declare const googleQueryOne: (googleSession: GoogleSession, request: RequestOne) => Promise<GoogleDriveFile>;
export declare class GoogleDriveFile extends FileQuery<GoogleDriveFile> implements FileMetadata {
    private googleSession;
    private fileData;
    id: string;
    link: string;
    name: string;
    constructor(googleSession: GoogleSession, fileData: FileData);
    bodyJson: <Body>() => Promise<Body>;
    bodyString: () => Promise<string>;
    moveTo: (fileMetadata: FileMetadata) => Promise<void>;
    rename: (name: string) => Promise<void>;
    create: (name: string, contentType: string, body: any) => Promise<FileMetadata>;
    intoSheet: () => GoogleSheet;
}
