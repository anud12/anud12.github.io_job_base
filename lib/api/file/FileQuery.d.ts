import { RequestList } from "./RequestList.type";
import { RequestOne } from "./RequestOne.type";
export type FileQueryClient<T> = {
    queryList: (request: RequestList) => Promise<Array<T>>;
    queryOne: (request: RequestOne) => Promise<T>;
    getId: () => string | undefined;
};
export declare class FileQuery<T> {
    private client;
    constructor(client: FileQueryClient<T>);
    findAll: () => Promise<T[]>;
    findByName: (name: string) => Promise<T[]>;
    findOneByName: (name: string) => Promise<T>;
    findOneById: (id: string) => Promise<T>;
}
