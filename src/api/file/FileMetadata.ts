export type FileMetadata = {
  create: <T extends FileMetadata>(name: string, contentType: string, body: any) => Promise<T>;
  id: string;
  name: string;
  link: string;
  bodyString: () => Promise<String>;
  bodyJson: <Body>() => Promise<Body>;
  moveTo: (fileMetadata: FileMetadata) => Promise<void>;
  rename: (name: string) => Promise<void>;
}