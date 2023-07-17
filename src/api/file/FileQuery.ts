import { RequestList } from "./RequestList.type";
import { RequestOne } from "./RequestOne.type";

export type FileQueryClient<T> = {
  queryList: (request: RequestList) => Promise<Array<T>>;
  queryOne: (request: RequestOne) => Promise<T>;
  getId: () => string | undefined
}

export abstract class FileQuery<T, TUninitialized> {
  constructor(protected client: FileQueryClient<T>) {
  }

  abstract findOneByIdLazy: (id:string) => TUninitialized;

  findAll = () => {
    return this.client.queryList({
      parent: this.client.getId()
    })
  }

  findByName = (name: string) => {
    return this.client.queryList({
      parent: this.client.getId(),
      name
    })
  }

  findOneByName = (name: string) => {
    return this.client.queryOne({
      parent: this.client.getId(),
      name
    })
  }

  findOneById = (id: string) => {
    return this.client.queryOne({
      parent: this.client.getId(),
      id
    })
  }
}