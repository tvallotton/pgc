import { ConfigService } from "../config/config.service.ts";
import { PGService } from "../pg/pg.service.ts";
import type { Catalog, Schema } from "./schema.types.ts";

export class EnumService {
  constructor(
    readonly configService: ConfigService,
    readonly pgService: PGService,
    readonly catalog: Catalog,
  ) {
  }

  async setTableBackedEnums() {
    const configuredEnums = this.configService.enums();

    for (const enum_ of configuredEnums) {
      if (typeof enum_ == "string") {
        await this.setTableBackedEnumFromName(enum_);
      } else {
        this.setTableBackedEnumFromObject(enum_);
      }
    }
  }

  private async setTableBackedEnumFromName(enumName: string) {
    const rows = await this.pgService.pg.query(
      `select * from ${this.sanitizeIdentifier(enumName)}`,
    ) as Record<string, string>[];
    const values = [];
    for (const row of rows) {
      for (const key in row as object) {
        values.push(row[key]);
        break;
      }
    }
    this.setTableBackedEnumFromObject({ [enumName]: values });
  }

  private setTableBackedEnumFromObject(
    enum_: Record<string, string[]>,
  ) {
    for (const enumFullName in enum_) {
      const [schemaName, name] = this.identifierParts(enumFullName);

      const values = enum_[enumFullName];
      const schema = this.findSchema(schemaName);
      if (!schema) return;

      schema.enums.push({ name, values });
      schema.records = schema.records.filter((table) => table.name != name);
    }
  }

  private findSchema(name: string): Schema | undefined {
    return this.catalog.schemas.find((schema) => schema.name == name);
  }

  private identifierParts(identifier: string): [string, string] {
    const [name, schema] = identifier.split(".").slice(0, 2).reverse();
    return [schema ?? "public", name];
  }

  private sanitizeIdentifier(identifier: string) {
    const [schema, name] = this.identifierParts(identifier).map((part) =>
      part.replaceAll(/"/g, '""')
    );
    return `"${schema}"."${name}"`;
  }
}
