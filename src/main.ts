import { PGlite } from "@electric-sql/pglite";

import { program } from "commander";
import init from "./init.ts";
import { ConfigService } from "./config/config.service.ts";
import { BuildService } from "./build/build.service.ts";

program.command("build").description(
  "build source code from SQL",
).option(
  "-f, --file <path>",
  "specify the path of the config file",
  "pgc.yaml",
).action(async (options) => {
  const { error, configService } = ConfigService.fromFilePath(options.file);

  if (!configService) {
    console.log(error);
    return;
  }
  let buildService;
  try {
    buildService = await BuildService.fromConfig(configService);
    await buildService.build();
  } catch (e) {
    console.log((e as Error).message);
  } finally {
    await buildService?.close();
  }
});

program.command("init").description("initialize a default pgc.yaml file")
  .option(
    "-f, --file <path>",
    "specify the path of the config file",
    "pgc.yaml",
  ).action((options) => {
    return init(options.file);
  });

program.version("1.0").parse();
