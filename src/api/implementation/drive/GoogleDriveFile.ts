import {FileMetadata} from "../../file/FileMetadata";
import {GoogleSession} from "../GoogleSession";
import {GoogleSheet} from "../sheet/GoogleSheet";
import {fetchGoogle} from "../fetchGoogle";
import {GoogleDriveFileData} from "./GoogleDriveFileData";
import {FileMetadataUninitialized} from "../../file/FileMetadataUninitialized";
import {GoogleDriveFileUninitialized} from "./GoogleDriveFileUninitialized";

export class GoogleDriveFile extends GoogleDriveFileUninitialized implements FileMetadata {
  id: string;
  link: string;
  name: string;

  constructor(private googleSession: GoogleSession, private fileData: GoogleDriveFileData) {
    super(googleSession, fileData.id);
    this.id = fileData.id;
    this.name = fileData.name;
    this.link = `https://drive.google.com/uc?id=${fileData.id}`;
  }

  bodyJson = async <Body>(): Promise<Body> => {
    console.log(`GoogleDriveFile[${this.id}].bodyJson()`);
    const response = await fetchGoogle(`https://www.googleapis.com/drive/v3/files/${this.fileData.id}?alt=media`, {
      headers: {
        "Authorization": `Bearer ${this.googleSession.token}`
      }
    })
    return (await response.json()) as Body
  }

  bodyString = async (): Promise<string> => {
    console.log(`GoogleDriveFile[${this.id}].bodyString()`);
    const response = await fetchGoogle(`https://www.googleapis.com/drive/v3/files/${this.fileData.id}?alt=media`, {
      headers: {
        "Authorization": `Bearer ${this.googleSession.token}`
      }
    })
    return (await response.text())
  }

  moveTo = async (fileMetadata: FileMetadata): Promise<void> => {
    console.log(`GoogleDriveFile[${this.id}].moveTo(fileMetadata:${fileMetadata.id})`);
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
    console.log(`GoogleDriveFile[${this.id}].rename(fileMetadata:${name})`);
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

  createLazy = async (name: string, contentType: string, body: any): Promise<FileMetadataUninitialized> => {
    console.log(`GoogleDriveFile[${this.id}].create(name:${name}, contentType:${contentType}`);
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
    return new GoogleDriveFileUninitialized(this.googleSession, json.id);
  }

  create = async (name: string, contentType: string, body: any): Promise<FileMetadata> =>  {
    const fileMetadataUninitialized = await this.createLazy(name, contentType, body)
    return this.findOneById(fileMetadataUninitialized.id);
  }

  intoSheet = (): GoogleSheet => {
    return new GoogleSheet(this.googleSession, this.fileData);
  }

  load = async (): Promise<FileMetadata> => {
    console.log(`GoogleDriveFile[${this.id}].load()`);
    return this;
  }

}