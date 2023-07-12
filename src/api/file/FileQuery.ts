import {RequestList} from "./RequestList.type";
import {RequestOne} from "./RequestOne.type";

export type FileQueryClient<T> = {
  queryList: (request: RequestList) => Promise<Array<T>>;
  queryOne: (request: RequestOne) => Promise<T>;
  getId: () => string | undefined
}

export class FileQuery<T> {
  constructor(private client: FileQueryClient<T>) {
  }

  findAll = () => {
    console.log(`FileQuery.findAll()`)
    return this.client.queryList({
      parent: this.client.getId()
    })
  }

  findByName = (name: string) => {
    console.log(`FileQuery.findAll(name:${name})`)
    return this.client.queryList({
      parent: this.client.getId(),
      name
    })
  }

  findOneByName = (name: string) => {
    console.log(`FileQuery.findOneByName(name:${name})`);
    return this.client.queryOne({
      parent: this.client.getId(),
      name
    })
  }
  findOneById = (id: string) => {
    console.log(`FileQuery.findOneById(id:${id})`);
    return this.client.queryOne({
      parent: this.client.getId(),
      id
    })
  }
}