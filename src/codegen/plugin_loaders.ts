import * as path from "node:path";
import * as os from "node:os";
import { defaultWasmBinary } from "./wasm_bin.ts";

interface PluginOptions {
  url?: string;
  sha256?: string;
  cacheDir: string;
}

export function loadPluginBinary(options: PluginOptions) {
  if (options.url === undefined) {
    return defaultWasmBinary();
  }

  if (
    options.url.startsWith("http://") || options.url.startsWith("https://")
  ) {
    return loadFromNetwork(options);
  } else {
    return Deno.readFile(options.url);
  }
}

async function loadFromURL(options: PluginOptions) {
  const response = await fetch(options.url!);
  const bytes = new Uint8Array(await response.arrayBuffer());
  cacheFile(options, bytes);
  return bytes;
}

function cacheFile(options: PluginOptions, bytes: Uint8Array) {
  if (options.sha256) {
    return Deno.writeFile(
      path.join(options.cacheDir, options.sha256),
      bytes,
    );
  }
}

function loadFromNetwork(options: PluginOptions) {
  if (options.sha256) {
    const plugin = loadFromCache(options);

    if (plugin) {
      return plugin;
    }
  }

  return loadFromURL(options);
}

function loadFromCache(options: PluginOptions) {
  try {
    if (options.sha256) {
      return Deno.readFile(path.join(options.cacheDir, options.sha256));
    }
  } catch (error) {
    if ((error as any).code == "ENOENT") {
      return;
    } else {
      throw error;
    }
  }
}
