import { FileQuery } from "../../file/FileQuery";
import { GoogleSession } from "../GoogleSession";
import { GoogleDriveFile } from "./GoogleDriveFile";
export declare class GoogleDrive extends FileQuery<GoogleDriveFile> {
    private googleSession;
    constructor(googleSession: GoogleSession);
}
