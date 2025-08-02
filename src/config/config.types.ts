import { z } from "zod";

const DatabaseURLSourceConfig = z.object({
  url: z.string().optional(),
  pglite: z.undefined(),
  migrations: z.undefined(),
});

const PGLiteOptions = z.object({
  username: z.string().optional(),
  database: z.string().optional(),
  extensions: z.record(z.string()),
});

const MigrationsSourceConfig = z.object({
  url: z.undefined(),
  migrations: z.string().array().or(z.string()),
  pglite: PGLiteOptions.optional(),
});

const DatabaseConfig = MigrationsSourceConfig.or(DatabaseURLSourceConfig);

const PluginConfig = z.object({
  url: z.string(),
  sha256: z.string().optional(),
});

const TypeOverride = z.object({
  name: z.string(),
  annotation: z.string().optional(),
  import: z.string().array().optional(),
});

const CodegenConfig = z.object({
  out: z.string(),
  target: z.string(),
  plugin: PluginConfig.optional(),
  types: z.record(z.string(), TypeOverride).optional(),
  options: z.object({}).passthrough().optional().nullable(),
});

export const Config = z.object({
  version: z.string(),
  queries: z.string().or(z.string().array()),
  cache_dir: z.string().optional(),
  disable_cache: z.boolean().default(false),
  database: DatabaseConfig,
  codegen: CodegenConfig,
});

export type Config = z.infer<typeof Config>;
