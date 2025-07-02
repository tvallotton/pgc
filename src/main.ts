import { PGlite } from "@electric-sql/pglite";

import { program } from "commander";
import init from "./init.ts";
import { ConfigService } from "./config/config.service.ts";
import { pg_trgm } from "@electric-sql/pglite/contrib/pg_trgm";
import { Glob, glob } from "glob";
import { PGService } from "./pg/pg.service.ts";
import { FileCollectorService } from "./fs/file_collector.service.ts";
import { RawQueryCollector } from "./query_collector/query_collector.servce.ts";
import { QueryParserService } from "./query_collector/query_parser.service.ts";
import { PGliteService } from "./pg/pglite.service.ts";
import { SchemaService } from "./schema_service/schema.service.ts";

program.command("build").description(
  "Build source code from SQL",
).option(
  "-f, --file <path>",
  "specify the path of the config file",
  "pgc.yaml",
).action(async (options) => {
  console.log("0");
  const { error, configService } = ConfigService.fromFilePath(options.file);

  if (!configService) {
    console.log(error);
    return;
  }

  const pgService = await PGService.fromConfig(configService);

  const fileCollectorService = new FileCollectorService(configService);

  const rawQueryCollector = new RawQueryCollector(fileCollectorService);

  const queryParser = new QueryParserService(pgService);

  const rawQueries = await rawQueryCollector.loadQueries();
  const schemaService = new SchemaService(
    pgService,
    fileCollectorService,
  );

  await schemaService.loadCatalog();

  for (const rawQuery of rawQueries) {
    console.log(await queryParser.parseQuery(rawQuery));
  }
});

program.command("init").description("Initialize a default pgc.yaml file")
  .option(
    "-f, --file <path>",
    "specify the path of the config file",
    "pgc.yaml",
  ).action((options) => {
    return init(options.file);
  });

program.version("1.0").parse();
