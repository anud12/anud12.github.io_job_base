import { FileQuery } from "../../file/FileQuery";
import { GoogleSession } from "../GoogleSession";
import {GoogleDriveFile} from "./GoogleDriveFile";
import {prepareRequest} from "./prepareRequest";
import {fetchGoogle} from "../fetchGoogle";
import {GoogleDriveFileData} from "./GoogleDriveFileData";
import {RequestOne} from "../../file/RequestOne.type";
import {RequestList} from "../../file/RequestList.type";
import {GoogleDriveFileUninitialized} from "./GoogleDriveFileUninitialized";


export const googleQueryList = async (googleSession: GoogleSession, request: RequestList): Promise<Array<GoogleDriveFile>> => {
  console.log(`googleQueryList(session, request:${JSON.stringify(request)})`);
  return (await prepareRequest(googleSession.token, request)).map((fileData) => new GoogleDriveFile(googleSession, fileData));
}
export const googleQueryOne = async (googleSession: GoogleSession, request: RequestOne): Promise<GoogleDriveFile> => {
  console.log(`googleQueryOne(session, request:${JSON.stringify(request)})`);
  let fileData: GoogleDriveFileData
  if (request.id) {
    const response = await fetchGoogle(`https://www.googleapis.com/drive/v3/files/${request.id}?fields=id, name, mimeType, parents`, {
      method: "GET", headers: {
        "Authorization": `Bearer ${googleSession.token}`
      }
    })
    fileData = await response.json()
  } else {
    const list = await prepareRequest(googleSession.token, {
      name: request.name,
      parent: request.parent,
      size: 1
    })
    if (list?.length !== 1) {
      throw "googleQueryOne returned list length different than 1"
    }
    fileData = list[0]
  }
  if (request.parent && !fileData.parents?.includes(request.parent)) {
    throw new Error(`googleQueryOne returned file has diferent parent than ${request.parent} : ${fileData}`);
  }

  if (request.name && fileData.name !== request.name) {
    throw new Error(`googleQueryOne returned file has diferent name than ${request.name} : ${fileData}`);
  }

  return new GoogleDriveFile(googleSession, fileData);
}

export class GoogleDrive extends FileQuery<GoogleDriveFile, GoogleDriveFileUninitialized> {
  constructor(protected googleSession: GoogleSession) {
    super({
      getId: () => undefined,
      queryList: (request) => googleQueryList(this.googleSession, request),
      queryOne: (request) => googleQueryOne(this.googleSession, request),
    });
  }

  findOneByIdLazy = (id: string): GoogleDriveFileUninitialized => {
    console.log(`GoogleDrive.findOneByIdLazy(id:${JSON.stringify(id)})`);

    return new GoogleDriveFileUninitialized(this.googleSession, id);
  }
}