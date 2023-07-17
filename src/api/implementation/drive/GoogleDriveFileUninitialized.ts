import {FileMetadata} from "../../file/FileMetadata";
import {GoogleSession} from "../GoogleSession";
import {FileMetadataUninitialized} from "../../file/FileMetadataUninitialized";
import {googleQueryList, googleQueryOne} from "./GoogleDrive";
import {FileQuery} from "../../file/FileQuery";
import {GoogleDriveFile} from "./GoogleDriveFile";

export class GoogleDriveFileUninitialized extends FileQuery<GoogleDriveFile, GoogleDriveFileUninitialized> implements FileMetadataUninitialized {

  constructor(protected googleSession: GoogleSession, public id: string) {
    super({
      getId: () => id,
      queryList: (request) => googleQueryList(this.googleSession, request),
      queryOne: (request) => googleQueryOne(this.googleSession, request),
    });
  }

  load(): Promise<FileMetadata> {
    return this.client.queryOne({
      id: this.id
    })
  }

  findOneByIdLazy = (id: string): GoogleDriveFileUninitialized => {
    return new GoogleDriveFileUninitialized(this.googleSession, id);
  }
}