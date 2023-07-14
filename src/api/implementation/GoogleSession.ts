import FormData from "form-data";
import jwt from "jsonwebtoken";
import { GoogleDrive } from "./drive/GoogleDrive";
import {fetchGoogle} from "./fetchGoogle";
export type GoogleSession = {
  token: string,
  expirationUnixSeconds: Date
  intoDrive: () => GoogleDrive
}
const getSession = async (): Promise<Pick<GoogleSession, "token" | "expirationUnixSeconds">> => {
  const privateKey = process.env.PRIVATE_KEY;
  if (!privateKey) {
    throw "PRIVATE_KEY not set"
  }
  const clientEmail = process.env.CLIENT_EMAIL;
  if (!clientEmail) {
    throw "CLIENT_EMAIL not set"
  }

  const expirationDate = new Date();
  expirationDate.setSeconds(expirationDate.getSeconds() + 3600);

  const claims = {
    iss: clientEmail,
    scope: "https://www.googleapis.com/auth/drive https://www.googleapis.com/auth/spreadsheets",
    aud: "https://oauth2.googleapis.com/token",
    exp: (expirationDate.getTime() / 1000),
    iat: (new Date().getTime() / 1000),
  }

  const token = jwt.sign(claims, privateKey, { algorithm: "RS256" });
  const body = new FormData();
  body.append("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");
  body.append("assertion", token);
  const response = await fetchGoogle("https://oauth2.googleapis.com/token", {
    method: "POST",
    body: body
  })
  const responseJson = (await response.json() as any);
  const access_token = responseJson.access_token;
  if (!access_token) {
    throw new Error(`Response doesnt contain access token, response: ${JSON.stringify(responseJson, null, 2)}`)
  }


  return {
    token: access_token,
    expirationUnixSeconds: expirationDate
  }
}

export const newGoogleSession = async ():Promise<GoogleSession> => {
  console.log(`newGoogleSession()`);
  const data = await getSession();
  return new class implements GoogleSession {
    token: string = data.token;
    expirationUnixSeconds: Date = data.expirationUnixSeconds;
    constructor() {
    }
    intoDrive = (): GoogleDrive => {
      return new GoogleDrive(this as GoogleSession)
    };

  }
}