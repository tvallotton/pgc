import { Buffer } from "node:buffer";

async function cargoBuild() {
  const args = ["build", "--target", "wasm32-unknown-unknown"];
  if (Deno.env.get("ENV") == "prod") {
    args.push("--release");
  }
  const cmd = new Deno.Command("cargo", { args, cwd: "./codegen" });

  await cmd.spawn().status;
}

const WASM_PATH = Deno.env.get("ENV") == "prod"
  ? `./codegen/target/wasm32-unknown-unknown/release/pgc-codegen.wasm`
  : `./codegen/target/wasm32-unknown-unknown/debug/pgc-codegen.wasm`;

async function getWasmBase64() {
  const wasmBytes = await Deno.readFile(WASM_PATH);

  const inputStream = new ReadableStream({
    start(controller) {
      controller.enqueue(wasmBytes);
      controller.close();
    },
  });

  const compressedStream = inputStream.pipeThrough(
    new CompressionStream("gzip"),
  );

  const compressedArrayBuffer = await new Response(compressedStream)
    .arrayBuffer();
  const compressedBytes = new Uint8Array(compressedArrayBuffer);

  return Buffer.from(compressedBytes).toString("base64");
}

async function main() {
  await cargoBuild();
  const base64 = await getWasmBase64();

  const template = `
import { Buffer } from "node:buffer";
export async function defaultWasmBinary() {
  const base64Bin = '${base64}';
  const bytes = new Uint8Array(Buffer.from(base64Bin, "base64"));

  const blob = new Blob([bytes], { type: "application/gzip" });
  const decompressedStream = blob.stream().pipeThrough(new DecompressionStream("gzip"));
  const decompressedBuffer = await new Response(decompressedStream).arrayBuffer();
  return new Uint8Array(decompressedBuffer);
}
  `;
  await Deno.writeTextFile(`src/codegen/wasm_bin.ts`, template);
}

export function defaultWasmPlugin() {
  const base64Bin = "${base64}";
  const bytes = new Uint8Array(Buffer.from(base64Bin, "base64"));

  const blob = new Blob([bytes], { type: "application/gzip" });
  const decompressedStream = blob.stream().pipeThrough(
    new DecompressionStream("gzip"),
  );

  return WebAssembly.instantiateStreaming(
    new Response(decompressedStream, {
      headers: {
        "Content-Type": "application/wasm",
      },
    }),
  );
}

await main();
