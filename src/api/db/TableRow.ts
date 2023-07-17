export type TableRow<IdType, DataType = any> = {
    id: IdType | undefined;
    data: DataType;
}