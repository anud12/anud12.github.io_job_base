import {FileQuery} from "../../file/FileQuery";
import {RequestList} from "../../file/RequestList.type";
import {RequestOne} from "../../file/RequestOne.type";
import {FileMetadata} from "../../file/FileMetadata";
import {GoogleSession} from "../GoogleSession";
import {FileData, prepareRequest} from "./prepareRequest";
import {GoogleSheet} from "../sheet/GoogleSheet";
import {fetchGoogle} from "../fetchGoogle";

export const googleQueryList = async (googleSession: GoogleSession, request: RequestList): Promise<Array<GoogleDriveFile>> => {
  console.log(`googleQueryList(session, request:${JSON.stringify(request)})`);
  return (await prepareRequest(googleSession.token, request)).map((fileData) => new GoogleDriveFile(googleSession, fileData));
}
export const googleQueryOne = async (googleSession: GoogleSession, request: RequestOne): Promise<GoogleDriveFile> => {
  console.log(`googleQueryOne(session, request:${JSON.stringify(request)})`);
  let fileData: FileData
  if (request.id) {
    const response = await fetchGoogle(`https://www.googleapis.com/drive/v3/files/${request.id}?fields=id, name, mimeType, parents`, {
      method: "GET", headers: {
        "Authorization": `Bearer ${googleSession.token}`
      }
    })
    fileData = await response.json()
  } else {
    const list = await prepareRequest(googleSession.token, {
      name:request.name,
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
export class GoogleDriveFile extends FileQuery<GoogleDriveFile> implements FileMetadata {
  id: string;
  link: string;
  name: string;
  constructor(private googleSession: GoogleSession, private fileData: FileData) {
    super({
      getId: () => this.id,
      queryList: (request) => googleQueryList(this.googleSession, request),
      queryOne: (request) => googleQueryOne(this.googleSession, request),
    });
    this.id = fileData.id;
    this.name = fileData.name;
    this.link = `https://drive.google.com/uc?id=${fileData.id}`;
  }

  bodyJson = async<Body>(): Promise<Body> => {
    console.log(`GoogleDriveFile.bodyJson()`);
    const response = await fetchGoogle(`https://www.googleapis.com/drive/v3/files/${this.fileData.id}?alt=media`, {
      headers: {
        "Authorization": `Bearer ${this.googleSession.token}`
      }
    })
    return (await response.json()) as Body
  }

  bodyString = async (): Promise<string> => {
    console.log(`GoogleDriveFile.bodyString()`);
    const response = await fetchGoogle(`https://www.googleapis.com/drive/v3/files/${this.fileData.id}?alt=media`, {
      headers: {
        "Authorization": `Bearer ${this.googleSession.token}`
      }
    })
    return (await response.text())
  }

  moveTo = async (fileMetadata: FileMetadata): Promise<void> => {
    console.log(`GoogleDriveFile.moveTo(fileMetadata:${fileMetadata.id})`);
    if (!this.fileData.parents) {
      throw "No parents found";
    }
    await fetchGoogle(`https://www.googleapis.com/upload/drive/v3/files/${this.id}?removeParents=${this.fileData.parents[0]}&addParents=${fileMetadata.id}`, {
      method: "PATCH",
      headers: {
        "Authorization": `Bearer ${this.googleSession.token}`
      }
    })
    return
  }
  rename = async (name: string): Promise<unknown> => {
    console.log(`GoogleDriveFile.rename(fileMetadata:${name})`);
    if (!this.fileData.parents) {
      throw "No parents found";
    }
    await fetchGoogle(`https://www.googleapis.com/upload/drive/v3/files/${this.id}}`, {
      method: "PATCH",
      headers: {
        "Authorization": `Bearer ${this.googleSession.token}`
      },
      body: JSON.stringify({
        name: name
      })
    })
    return;
  }

  create = async (name: string, contentType: string, body: any): Promise<FileMetadata> => {
    console.log(`GoogleDriveFile.create(name:${name}, contentType:${contentType}`);
    const resumable_req = await fetchGoogle("https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable", {
      method: "POST",
      headers: {
        "Authorization": `Bearer ${this.googleSession.token}`,
        "Content-Type": "application/json; charset=UTF-8",
      },
      body: JSON.stringify({
        "name": name,
        "parents": [this.fileData.id],
        "mimeType": contentType,
      }),
    });

    const location = resumable_req.headers.get("Location")!;

    const put_req = await fetchGoogle(location, {
      method: "PUT",
      headers: {
        "Authorization": `Bearer ${this.googleSession.token}`,
        "Content-Type": contentType,
      },
      body: body,
    });

    const json = await put_req.json();
    return this.findOneById(json.id);
  }
  intoSheet = (): GoogleSheet => {
    return new GoogleSheet(this.googleSession, this.fileData);
  }

}