import * as YAML from "jsr:@std/yaml";
import { Config } from "./config.types.ts";
import type { File } from "../fs/fs.types.ts";
import * as path from "node:path";
import * as os from "node:os";

import type { Extension, PGliteOptions } from "@electric-sql/pglite";

export class ConfigService {
  constructor(readonly config: Config, readonly file: File) {}

  static fromFilePath(path: string) {
    const content = Deno.readTextFileSync(path);
    return this.fromFile({ path, content });
  }

  static fromFile(file: File) {
    let config;
    if (file.path.endsWith(".json")) {
      config = JSON.parse(file.content);
    } else {
      config = YAML.parse(file.content);
    }
    const { data, error } = Config.safeParse(config);

    if (data) {
      return { configService: new ConfigService(data, file) };
    } else if (error) {
      return { error: ConfigService.formatErrorMessage(error, file.path) };
    }
    throw Error("unreachable");
  }

  static formatErrorMessage(
    error: ReturnType<typeof Config.safeParse>["error"],
    configFile: string,
  ) {
    for (const issue of error?.errors ?? []) {
      const code = issue.code.replaceAll("_", " ").replaceAll("union", "value");
      const path = issue.path.join(".");
      const message = issue.message.toLowerCase();
      return `error: ${code} for option "${path}", ${message}, at "${configFile}".`;
    }
  }

  enums(): (string | Record<string, string[]>)[] {
    const enums = this.config.codegen.enums;
    if (enums instanceof Array) {
      return enums;
    }
    return [];
  }

  queries() {
    if (typeof this.config.queries == "string") {
      return [this.config.queries];
    }
    return this.config.queries;
  }

  cacheDir() {
    if (os.platform() === "win32") {
      return path.join(
        Deno.env.get("LOCALAPPDATA") ??
          path.join(os.homedir(), "AppData", "Local"),
        "pgc",
      );
    }
    return (
      this.config.cache_dir ??
        path.join(os.homedir(), ".cache/pgc")
    );
  }

  migrations(): string[] {
    const { migrations } = this.config.database;
    if (migrations == undefined) return [];
    if (typeof migrations == "string") return [migrations];
    return migrations;
  }

  checkVersion() {
    const allowedVersions = ["1", "1.0", "1.0.0", "1.", "1.0."];
    if (!allowedVersions.includes(this.config.version)) {
      throw Error(
        `The version specified ("${this.config?.version}") is not supported by this CLI, you might need to update it.`,
      );
    }
  }
}
