import { PGliteService } from "./pglite.service.ts";
import type { PGSourceService, QueryDescription } from "./pg.types.ts";
import type { ConfigService } from "../config/config.service.ts";
import { PostgresDriverService } from "./postgres_driver.service.ts";
import type { RawQuery, SQLType } from "../query_collector/query.types.ts";

export class PGService {
  types: Map<number, SQLType>;
  constructor(readonly pg: PGSourceService) {
    this.types = new Map();
  }

  static async fromConfig(configService: ConfigService) {
    const { database_url } = configService.config.schema;

    if (database_url != undefined) {
      const pg = PostgresDriverService.fromDatabaseUrl(database_url);
      return new PGService(pg).loadTypes();
    }

    const pg = await PGliteService.fromConfig(configService);
    return new PGService(pg).loadTypes();
  }

  async describe(query: RawQuery): Promise<QueryDescription> {
    const description = await this._describe(query);

    const inputs = description.inputs.map((id) => this.types.get(id)!);
    const outputs = description.outputs.map(({ type, name }) => ({
      name,
      type: this.types.get(type)!,
    }));

    return { ...description, inputs, outputs };
  }

  private async _describe(rawQuery: RawQuery) {
    try {
      return await this.pg.describe(rawQuery.sql);
    } catch (e) {
      throw Error(`\`${rawQuery.file.path}:${rawQuery.line}\` ${e}`);
    }
  }

  async loadTypes() {
    const types = await this.pg.query<SQLType>(`
      SELECT
          n.nspname AS schema,
          t.typname AS name,
          t.oid AS id
      FROM pg_type t
      LEFT JOIN pg_namespace n ON n.oid = t.typnamespace
      ORDER BY id;
    `);
    console.log("TYPES", types);
    for (const type of types) {
      this.types.set(type.id, type);
    }
    return this;
  }
}
