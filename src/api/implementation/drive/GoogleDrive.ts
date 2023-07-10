import {FileQuery} from "../../file/FileQuery";
import {GoogleDriveFile, googleQueryList, googleQueryOne} from "./GoogleDriveFile";

export class GoogleDrive extends FileQuery<GoogleDriveFile> {
  constructor() {
    super({
      getId: () => undefined,
      queryList: googleQueryList,
      queryOne: googleQueryOne,
    });
  }
}