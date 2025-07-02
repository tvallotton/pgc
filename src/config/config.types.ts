import { z } from "zod";

const DatabaseURLSourceConfig = z.object({
  database_url: z.string(),
  pglite: z.undefined(),
  migrations: z.undefined(),
});

const PGLiteOptions = z.object({
  username: z.string().optional(),
  database: z.string().optional(),
  extensions: z.record(z.string()),
});

const MigrationsSourceConfig = z.object({
  database_url: z.undefined(),
  migrations: z.string().array().or(z.string()),
  pglite: PGLiteOptions.optional(),
});

const SchemaConfig = MigrationsSourceConfig.or(DatabaseURLSourceConfig);

const PluginConfig = z.object({
  url: z.string(),
  sha256: z.string().optional(),
});

const CodegenConfig = z.object({
  out: z.string(),
  plugin: PluginConfig.optional(),
  options: z.object({}).passthrough().optional().nullable(),
});

export const Config = z.object({
  version: z.string(),
  queries: z.string().or(z.string().array()),
  cache_dir: z.string().optional(),
  disable_cache: z.boolean().default(false),
  schema: SchemaConfig,
  codegen: CodegenConfig.optional(),
  env_file: z.string().or(z.string().array()),
});

export type Config = z.infer<typeof Config>;
