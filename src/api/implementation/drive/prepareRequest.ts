import { RequestList } from "../../file/RequestList.type";
import { fetchGoogle } from "../fetchGoogle";
import { GoogleDriveFileData } from "./GoogleDriveFileData";

export const prepareRequest = async (token: string, requestList: RequestList): Promise<Array<GoogleDriveFileData>> => {
    let query: Array<string> = [];
    if (requestList.name) {
        query = [...query, `name='${requestList.name}'`];
    }

    if (requestList.parent) {
        query = [...query, `'${requestList.parent}' in parents`];
    }

    const queryString = query.join(" and ");
    let url = new URL("https://www.googleapis.com/drive/v3/files");
    url.searchParams.set("fields", "files(id, name, mimeType, parents)");
    url.searchParams.set("q", queryString);

    const response = await fetchGoogle(url, {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${token}`
        }
    });
    return (await response.json()).files;
}