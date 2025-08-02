import postgres from "postgres";
import { QueryDescription } from "./pg.types.ts";

export class PostgresDriverService {
  constructor(
    readonly sql: postgres.Sql,
  ) {}

  static new(databaseUrl: string) {
    if (databaseUrl.startsWith("$")) {
      return PostgresDriverService.fromEnvironment(databaseUrl.slice(1));
    }
    return PostgresDriverService.fromDatabaseUrl(databaseUrl);
  }

  static fromDatabaseUrl(databaseUrl: string) {
    const sql = postgres(databaseUrl);
    return new PostgresDriverService(sql);
  }

  static fromEnvironment(variableName: string) {
    return PostgresDriverService.fromDatabaseUrl(Deno.env.get(variableName)!);
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
  async close() {
    await this.sql.end();
  }
}

function databaseUrl(databaseUrl: string) {
  if (databaseUrl.startsWith("$")) {
    return;
  }
}
