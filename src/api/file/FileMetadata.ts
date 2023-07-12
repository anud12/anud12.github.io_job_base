export type FileMetadata = {
  create: (name: string, contentType: string, body: any) => Promise<FileMetadata>;
  id: string;
  name: string;
  link: string;
  bodyString: () => Promise<string>;
  bodyJson: <Body>() => Promise<Body>;
  moveTo: (fileMetadata: FileMetadata) => Promise<void>;
  rename: (name: string) => Promise<void>;
}