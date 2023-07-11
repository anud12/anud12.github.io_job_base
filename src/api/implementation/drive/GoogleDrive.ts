import {FileQuery} from "../../file/FileQuery";
import {GoogleSession} from "../GoogleSession";
import {GoogleDriveFile, googleQueryList, googleQueryOne} from "./GoogleDriveFile";

export class GoogleDrive extends FileQuery<GoogleDriveFile> {
  constructor(private googleSession: GoogleSession) {
    super({
      getId: () => undefined,
      queryList: (request) => googleQueryList(this.googleSession, request),
      queryOne: (request) => googleQueryOne(this.googleSession, request),
    });
  }
}