import { FileMetadata } from "../../file/FileMetadata";
import { GoogleSession } from "../GoogleSession";
import { GoogleSheet } from "../sheet/GoogleSheet";
import { GoogleDriveFileData } from "./GoogleDriveFileData";
import { FileMetadataUninitialized } from "../../file/FileMetadataUninitialized";
import { GoogleDriveFileUninitialized } from "./GoogleDriveFileUninitialized";
export declare class GoogleDriveFile extends GoogleDriveFileUninitialized implements FileMetadata {
    protected googleSession: GoogleSession;
    private fileData;
    id: string;
    link: string;
    name: string;
    constructor(googleSession: GoogleSession, fileData: GoogleDriveFileData);
    bodyJson: <Body>() => Promise<Body>;
    bodyString: () => Promise<string>;
    moveTo: (fileMetadata: FileMetadata) => Promise<void>;
    rename: (name: string) => Promise<unknown>;
    createLazy: (name: string, contentType: string, body: any) => Promise<FileMetadataUninitialized>;
    create: (name: string, contentType: string, body: any) => Promise<FileMetadata>;
    intoSheet: () => GoogleSheet;
    load: () => Promise<FileMetadata>;
}
