type Data = Record<string, string>;
export type TableRow<IdType> = {
    id: IdType | undefined;
    data: Data;
};
export {};
