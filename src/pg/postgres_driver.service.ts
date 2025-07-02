import postgres from "postgres";
import { QueryDescription } from "./pg.types.ts";

export class PostgresDriverService {
  constructor(
    readonly sql: postgres.Sql,
  ) {}

  static fromDatabaseUrl(databaseUrl: string) {
    const sql = postgres(databaseUrl);
    return new PostgresDriverService(sql);
  }

  async query<T>(query: string) {
    const rows = [...await this.sql.unsafe(query)];
    return rows as T[];
  }
  async execute(query: string) {
    await this.sql.unsafe(query).execute();
  }
  async describe(query: string): Promise<QueryDescription<number>> {
    const { types, columns } = await this.sql.unsafe(query).describe();

    return {
      query,
      inputs: types,
      outputs: columns.map((column) => ({
        name: column.name,
        type: column.type,
      })),
    };
  }
}
