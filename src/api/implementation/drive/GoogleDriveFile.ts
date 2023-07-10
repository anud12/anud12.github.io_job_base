import {FileQuery} from "../../file/FileQuery";
import {RequestList} from "../../file/RequestList.type";
import {RequestOne} from "../../file/RequestOne.type";
import {FileMetadata} from "../../file/FileMetadata";

export const googleQueryList =async (request:RequestList):Promise<Array<GoogleDriveFile>> => {

}
export const googleQueryOne = async (request:RequestOne):Promise<GoogleDriveFile> => {

}
export class GoogleDriveFile extends FileQuery<GoogleDriveFile> implements FileMetadata {
  id: string;
  link: string;
  constructor() {
    super({
      getId: () => this.id,
      queryList:googleQueryList,
      queryOne:googleQueryOne,
    });
  }

  bodyJson<Body>(): Promise<Body> {
    return Promise.resolve(undefined);
  }

  bodyString(): Promise<String> {
    return Promise.resolve("");
  }

  create<T>(name: string, contentType: string, body: any): Promise<T> {
    return Promise.resolve(undefined);
  }

  moveTo(fileMetadata: FileMetadata): Promise<void> {
    return Promise.resolve(undefined);
  }

  name: string;

  rename(name: string): Promise<void> {
    return Promise.resolve(undefined);
  }
}