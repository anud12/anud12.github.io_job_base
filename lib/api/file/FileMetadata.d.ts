import { FileMetadataUninitialized } from "./FileMetadataUninitialized";
export type FileMetadata = {
    createLazy: (name: string, contentType: string, body: any) => Promise<FileMetadataUninitialized>;
    create: (name: string, contentType: string, body: any) => Promise<FileMetadata>;
    id: string;
    name: string;
    link: string;
    bodyString: () => Promise<string>;
    bodyJson: <Body>() => Promise<Body>;
    moveTo: (fileMetadata: FileMetadata) => Promise<unknown>;
    rename: (name: string) => Promise<unknown>;
};
