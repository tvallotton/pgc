import { ConfigService } from "../config/config.service.ts";
import type { FileCollectorService } from "../fs/file_collector.service.ts";
import type { PGService } from "../pg/pg.service.ts";
import type { Catalog } from "./schema.types.ts";

export class SchemaService {
  constructor(
    readonly pgService: PGService,
    readonly fileCollectorService: FileCollectorService,
    readonly configService: ConfigService,
  ) {}

  async loadCatalog() {
    await this.loadMigrations();
    return this.querySchemas();
  }

  private async loadMigrations() {
    const migrationFiles = await this.fileCollectorService.getMigrationFiles();
    await this.pgService.loadMigrations(migrationFiles);
  }

  private async querySchemas() {
    const query = await this.pgService.pg.query(LOAD_SCHEMA_QUERY) as any;

    return query[0]["result"] as Catalog;
  }

  private async setTableEnums() {
    const enums = this.configService.enums();

    for (const enum_ of enums) {
      if (typeof enum_ == "string") {
      }
    }
  }
}

const LOAD_SCHEMA_QUERY = `
  WITH enums AS (
      SELECT
          n.nspname AS enum_schema,
          t.typname AS enum_name,
          array_agg(e.enumlabel ORDER BY e.enumsortorder) AS enum_values
      FROM pg_type t
      JOIN pg_enum e ON t.oid = e.enumtypid
      JOIN pg_namespace n ON n.oid = t.typnamespace
      WHERE n.nspname NOT IN ('pg_catalog', 'information_schema')
      GROUP BY n.nspname, t.typname
  ),

  pk_cols AS (
      SELECT
          conrelid,
          unnest(conkey) AS attnum
      FROM pg_constraint
      WHERE contype = 'p'
  ),

  uniq_cols AS (
      SELECT
          conrelid,
          unnest(conkey) AS attnum
      FROM pg_constraint
      WHERE contype = 'u'
  ),

  fk_cols AS (
      SELECT
          conrelid,
          unnest(conkey) AS attnum,
          confrelid,
          confkey[1] AS conf_attnum -- assuming single-column FK
      FROM pg_constraint
      WHERE contype = 'f'
  ),

  columns AS (
      SELECT
          n.nspname AS schema_name,
          c.relname AS table_name,
          a.attname AS column_name,
          jsonb_build_object(
             'name', case when  t.typcategory = 'A' then te.typname else t.typname end,
             'schema_name', case when  t.typcategory = 'A' then ne.nspname else tn.nspname end,
             'display', format_type(t.oid, NULL),
             'is_composite', (t.typtype = 'c'),
             'is_array', (t.typcategory = 'A'),
             'array_dimensions', a.attndims
          ) as type,
          pg_get_expr(ad.adbin, ad.adrelid) AS default_value,
          a.attnotnull = false AS is_nullable,
          (pk.conrelid IS NOT NULL) AS is_primary_key,
          (uq.conrelid IS NOT NULL) AS is_unique,
          (fk.conrelid IS NOT NULL) AS is_foreign_key,
          n2.nspname AS foreign_table_schema,
          c2.relname AS foreign_table_name,
          c.relkind
      FROM pg_class c
      JOIN pg_namespace n ON n.oid = c.relnamespace
      JOIN pg_attribute a ON a.attrelid = c.oid
      JOIN pg_type t ON t.oid = a.atttypid
      LEFT JOIN pg_attrdef ad ON ad.adrelid = c.oid AND ad.adnum = a.attnum
      LEFT JOIN pk_cols pk ON pk.conrelid = c.oid AND pk.attnum = a.attnum
      LEFT JOIN uniq_cols uq ON uq.conrelid = c.oid AND uq.attnum = a.attnum
      LEFT JOIN fk_cols fk ON fk.conrelid = c.oid AND fk.attnum = a.attnum
      LEFT JOIN pg_class c2 ON c2.oid = fk.confrelid
      LEFT JOIN pg_namespace n2 ON n2.oid = c2.relnamespace
      JOIN pg_namespace tn ON tn.oid = t.typnamespace
      LEFT JOIN pg_type te ON te.oid = t.typelem
      LEFT JOIN pg_namespace ne ON ne.oid = te.typnamespace
      WHERE c.relkind IN ('r', 'v', 'm', 'c')
        AND a.attnum > 0
        AND n.nspname NOT IN ('pg_catalog', 'information_schema', 'pg_toast')
        AND n.nspname NOT LIKE 'pg_temp_%'
  )

  SELECT jsonb_build_object(
      'schemas', jsonb_agg(
          jsonb_build_object(
              'name', schemas.schema_name,
              'enums', (
                  SELECT coalesce(jsonb_agg(
                      jsonb_build_object(
                          'name', enum_name,
                          'values', enum_values
                      )
                  ), '[]'::jsonb)
                  FROM enums
                  WHERE enums.enum_schema = schemas.schema_name
              ),
              'models', (
                  SELECT jsonb_agg(
                      jsonb_build_object(
                          'name', table_name,
                          'kind', case
                            when relkind = 'r' then 'table'
                            when relkind = 'c' then 'composite'
                            when relkind = 'v' then 'view'
                            when relkind = 'm' then 'materialized view'
                          end,
                          'columns', coalesce((
                              SELECT jsonb_agg(
                                  jsonb_build_object(
                                      'name', column_name,
                                      'type', type,
                                      'is_nullable', is_nullable,
                                      'default', default_value,
                                      'is_unique', is_unique,
                                      'is_primary_key', is_primary_key,
                                      'is_foreign_key', is_foreign_key,
                                      'foreign_table_schema', foreign_table_schema,
                                      'foreign_table_name', foreign_table_name
                                  )
                              )
                              FROM columns c2
                              WHERE c2.schema_name = schemas.schema_name
                                AND c2.table_name = t.table_name
                          ), '[]'::jsonb)
                      )
                  )
                  FROM (
                      SELECT DISTINCT table_name, relkind
                      FROM columns
                      WHERE schema_name = schemas.schema_name
                  ) t
              )
          )
      )
  ) AS result
  FROM (
      SELECT DISTINCT schema_name
      FROM columns
  ) schemas
`;
