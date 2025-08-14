import type { ConfigService } from "../config/config.service.ts";
import type { CodegenResponse } from "./condgen.types.ts";
import * as path from "jsr:@std/path";
import * as fs from "jsr:@std/fs";
import { loadPluginBinary } from "./plugin_loaders.ts";

export class CodegenService {
  constructor(readonly configService: ConfigService) {
  }

  async generate(payload: object) {
    const { error, files } = await this.runWasmCodegenModule(
      payload,
    );
    if (!files) {
      throw Error(`${error}`);
    }

    const outDir = path.join(this.configService.config.codegen?.out, "/");

    for (const file of files) {
      const filePath = path.join(outDir, file.path);

      if (!filePath.startsWith(outDir)) {
        throw Error(
          `got invalid path from plugin "${file.path}". Cannot create a file outside \`${outDir}\`.`,
        );
      }

      fs.ensureFile(filePath).then(() => {
        Deno.writeTextFile(filePath, file.content);
      });
    }
  }

  async clearDirectory(dirPath: string) {
    for await (const dirEntry of Deno.readDir(dirPath)) {
      const entryPath = path.join(dirPath, dirEntry.name);
      if (!dirEntry.isDirectory) {
        await Deno.remove(entryPath);
      } else if (dirEntry.isDirectory) {
        await this.clearDirectory(entryPath);
        await Deno.remove(entryPath);
      }
    }
  }

  async runWasmCodegenModule(payload: object) {
    const utf8JsonPayload = await this.serializePayload(payload);
    const { instance } = await this.loadPlugin();
    const exports = instance.exports as any;
    let requestPtr = exports.alloc(utf8JsonPayload.length);

    let mem = new Uint8Array((instance.exports.memory as any).buffer);
    mem.set(utf8JsonPayload, requestPtr);

    const responsePtr = exports.build(
      requestPtr,
      utf8JsonPayload.length,
    );
    const responseLength = exports.response_length();

    const slice = new Uint8Array(
      exports.memory.buffer,
      responsePtr,
      Number(responseLength),
    );

    return JSON.parse(new TextDecoder().decode(slice, {})) as CodegenResponse;
  }

  serializePayload(payload: object) {
    const jsonPayload = JSON.stringify(payload);
    const utf8JsonPayload = new TextEncoder().encode(jsonPayload);
    return utf8JsonPayload;
  }

  async loadPluginFromFile(url: string) {
    const path = url.replace(/^file:\/\//, "");
    const bytes = await Deno.readFile(path);
    return {
      bytes,
      instance: await WebAssembly.instantiate(bytes),
    };
  }

  async loadPlugin() {
    const cacheDir = this.configService.cacheDir();
    const { plugin } = this.configService.config.codegen;
    const binary = await loadPluginBinary({ ...plugin, cacheDir });

    if (plugin?.sha256) {
      await this.verifyChecksum(binary, plugin.sha256);
    } else if (plugin) {
      await this.warnMissingChecksum(binary);
    }

    return await WebAssembly.instantiate(binary);
  }

  async warnMissingChecksum(bytes: Uint8Array) {
    console.log(
      `You are missing a sha256 checksum in the use of a codegen plugin. ` +
        `To prevent unintended breakage set sha256: ${await this
          .computeChecksum(bytes)}`,
    );
  }
  async verifyChecksum(bytes: Uint8Array, sha256: string) {
    const digestHex = await this.computeChecksum(bytes);
    if (digestHex !== sha256!.toLowerCase()) {
      throw new Error(
        `checksum mismatch for codgen plugin, got ${digestHex}, expected ${sha256}`,
      );
    }
  }

  async computeChecksum(bytes: Uint8Array) {
    const digestBuffer = await crypto.subtle.digest("SHA-256", bytes);
    const digestArray = Array.from(new Uint8Array(digestBuffer));
    return digestArray.map((b) => b.toString(16).padStart(2, "0"))
      .join("");
  }
}
