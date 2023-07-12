import { GoogleDrive } from "./drive/GoogleDrive";
export type GoogleSession = {
    token: string;
    expirationUnixSeconds: Date;
    intoDrive: () => GoogleDrive;
};
export declare const newGoogleSession: () => Promise<{
    token: string;
    expirationUnixSeconds: Date;
    intoDrive: () => GoogleDrive;
}>;
