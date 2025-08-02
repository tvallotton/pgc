export interface PGSourceService {
  describe(query: string): Promise<QueryDescription<number>>;
  query<T>(query: string): Promise<T[]>;
  execute(query: string): Promise<void>;
  close(): Promise<void>;
}
export interface PGType {
  id: number;
  schema: string;
  name: string;
}

export interface Column<Type> {
  name: string;
  type: Type;
}

export interface QueryDescription<Type = PGType> {
  query: string;
  inputs: Type[];
  outputs: Column<Type>[];
}
