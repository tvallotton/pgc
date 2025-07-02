import { globIterate } from "glob";

import { Glob, glob } from "glob";
import type { File } from "./fs.types.ts";
import type { ConfigService } from "../config/config.service.ts";

export class FileCollectorService {
  constructor(readonly configService: ConfigService) {
  }

  getQueryFiles() {
    return this.getSQLFiles(this.configService.queries());
  }

  getMigrationFiles() {
    return this.getSQLFiles(this.configService.migrations());
  }

  async getSQLFiles(patterns: string[]) {
    const promises = [];

    for (const path of await this.filePaths(patterns)) {
      const promise: Promise<File> = Deno.readTextFile(path).then((
        content,
      ) => ({
        path,
        content,
      }));

      promises.push(promise);
    }

    return promises;
  }

  async filePaths(patterns: string[]) {
    const set = new Set<string>();

    for (const pattern of patterns) {
      for await (const path of globIterate(pattern)) {
        set.add(path);
      }
    }

    return set;
  }
}
