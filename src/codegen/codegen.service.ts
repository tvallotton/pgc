import type { ConfigService } from "../config/config.service.ts";
import type { CodegenResponse } from "./condgen.types.ts";
import * as path from "jsr:@std/path";
import * as fs from "jsr:@std/fs";
export class CodegenService {
  constructor(readonly configService: ConfigService) {
  }

  async generate(payload: object) {
    Deno.writeTextFile("catalog.json", JSON.stringify(payload));
    const { error, files } = await this.runWasmCodegenModule(
      payload,
    );
    if (!files) {
      throw Error(`${error}`);
    }

    console.log("FILES", files);

    const outDir = path.join(this.configService.config.codegen?.out, "/");

    for (const file of files) {
      const filePath = path.join(outDir, file.path);

      if (!filePath.startsWith(outDir)) {
        throw Error(
          `got invalid path from plugin \`${file.path}\`. Cannot create a file outside \`${outDir}\`.`,
        );
      }

      fs.ensureFile(filePath).then(() => {
        Deno.writeTextFile(filePath, file.content);
      });
    }
  }

  async runWasmCodegenModule(payload: object) {
    const utf8JsonPayload = await this.serializePayload(payload);
    const instance = await this.loadWasmInstance();
    const exports = instance.exports as any;
    let requestPtr = exports.alloc(utf8JsonPayload.length);

    let mem = new Uint8Array((instance.exports.memory as any).buffer);
    mem.set(utf8JsonPayload, requestPtr);

    const responsePtr = exports.build(
      requestPtr,
      utf8JsonPayload.length,
    );
    const responseLength = exports.response_length();
    console.log(responseLength);
    const slice = new Uint8Array(
      exports.memory.buffer,
      responsePtr,
      Number(responseLength),
    );

    console.log("response", new TextDecoder("utf-8").decode(slice, {}));

    return JSON.parse(new TextDecoder().decode(slice, {})) as CodegenResponse;
  }

  async serializePayload(payload: object) {
    const jsonPayload = JSON.stringify(payload);
    const utf8JsonPayload = new TextEncoder().encode(jsonPayload);
    return utf8JsonPayload;
  }

  async loadWasmInstance() {
    const plugin = this.configService.config.codegen?.plugin;

    if (plugin && plugin.sha256) {
      return this.loadPluginWithChecksum(plugin.url, plugin.sha256);
    }

    if (plugin) {
      return WebAssembly.instantiateStreaming(fetch(plugin.url)).then((
        { instance },
      ) => instance);
    }

    return this.loadDefaultWasmInstance();
  }

  async loadPluginWithChecksum(url: string, sha256: string) {
    const response = await fetch(url);
    const bytes = await response.arrayBuffer();

    // Compute SHA-256 hash
    const digestBuffer = await crypto.subtle.digest("SHA-256", bytes);
    const digestArray = Array.from(new Uint8Array(digestBuffer));
    const digestHex = digestArray.map((b) => b.toString(16).padStart(2, "0"))
      .join("");

    if (digestHex !== sha256!.toLowerCase()) {
      throw new Error(
        `checksum mismatch for codgen plugin, got ${digestHex}, expected ${sha256}`,
      );
    }

    const { instance } = await WebAssembly.instantiate(bytes);
    return instance;
  }

  async loadDefaultWasmInstance() {
    const file = await Deno.readFile(
      "./codegen/target/wasm32-unknown-unknown/debug/pgc-codegen.wasm",
    );

    const module = await WebAssembly.compile(
      file,
    );

    return await WebAssembly.instantiate(module);
  }
}
