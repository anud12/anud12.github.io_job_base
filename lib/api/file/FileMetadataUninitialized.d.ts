import { FileMetadata } from "./FileMetadata";
export type FileMetadataUninitialized = {
    id: string;
    load: () => Promise<FileMetadata>;
};
