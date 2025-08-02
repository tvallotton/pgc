import { globIterate } from "glob";
import type { ConfigService } from "./config/config.service.ts";
import type { Config } from "./config/config.types.ts";
import { Glob, glob } from "glob";
import type { File } from "./fs/fs.types.ts";

class FileReaderService {
  constructor(readonly configService: ConfigService) {
  }

  // async read(): File[] {
  //   for (const path of await this.filePaths()) {
  //     const contents = await Deno.readTextFile(path);
  //   }
  // }

  async filePaths() {
    const set = new Set<string>();

    for (const pattern of this.configService.queries()) {
      for await (const path of globIterate(pattern)) {
        set.add(path);
      }
    }
    return set;
  }
}
