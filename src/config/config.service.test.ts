import { ConfigService } from "./config.service.ts";

const YAML = `version: "1"
schema:
  migrations: ./migrations/*.sql
  pglite:
    username: postgres
    database: database_name
    extensions:
      vector: "@electric-sql/pglite/vector"
queries:
  - "*.sql"
codegen:
  binary:
    url: <url>.wasm
    sha256: <hex digest>
  out: ./src/queries
  options:
`;

Deno.test(function parseYAML() {
  ConfigService.fromFile({
    path: `pgc.yaml`,
    content: YAML,
  });
});
