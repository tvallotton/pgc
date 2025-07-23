import { type Extension, PGlite } from "@electric-sql/pglite";
import type { Config } from "../config/config.types.ts";
import { ConfigService } from "../config/config.service.ts";
import type { QueryDescription } from "./pg.types.ts";

export class PGliteService {
  constructor(readonly pg: PGlite, readonly configService: ConfigService) {
    new PGlite();
  }

  static async fromConfig(configService: ConfigService) {
    const options = configService.config.database.pglite;

    const extensions = await PGliteService.loadExtensions(options?.extensions);

    const pg = new PGlite({ ...options, extensions });

    return new PGliteService(pg, configService);
  }

  static async loadExtensions(extensionOptions?: Record<string, string>) {
    const extensions: Record<string, Extension> = {};
    for (const name in extensionOptions) {
      const module = await import(extensionOptions[name]);
      extensions[name] = module[name];
    }
    return extensions;
  }

  async query<T>(query: string) {
    const { rows } = await this.pg.query(query);
    console.log(rows);
    return rows as T[];
  }

  async execute(query: string) {
    await this.pg.exec(query);
  }

  async describe(query: string): Promise<QueryDescription<number>> {
    const { resultFields, queryParams } = await this.pg.describeQuery(query);
    console.log(query, resultFields, queryParams);
    return {
      query,
      inputs: queryParams.map((input) => input.dataTypeID),
      outputs: resultFields.map((column) => ({
        type: column.dataTypeID,
        name: column.name,
      })),
    };
  }
}
