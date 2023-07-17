import { FileMetadata } from "../../file/FileMetadata";
import { GoogleSession } from "../GoogleSession";
import { FileMetadataUninitialized } from "../../file/FileMetadataUninitialized";
import { FileQuery } from "../../file/FileQuery";
import { GoogleDriveFile } from "./GoogleDriveFile";
export declare class GoogleDriveFileUninitialized extends FileQuery<GoogleDriveFile, GoogleDriveFileUninitialized> implements FileMetadataUninitialized {
    protected googleSession: GoogleSession;
    id: string;
    constructor(googleSession: GoogleSession, id: string);
    load(): Promise<FileMetadata>;
    findOneByIdLazy: (id: string) => GoogleDriveFileUninitialized;
}
