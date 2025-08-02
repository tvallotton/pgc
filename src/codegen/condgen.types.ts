interface OutputFile {
  content: string;
  path: string;
}

export type CodegenResponse = {
  files: OutputFile[];
  error: undefined;
} | { files: undefined; error: string };

interface WasmPlugin {
  instance: WebAssembly.Instance;
  sha256: string;
}

interface WasmPluginLoader {
  load(): void;
  instance(): WebAssembly.Instance;
}
