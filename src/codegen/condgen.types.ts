interface OutputFile {
  content: string;
  path: string;
}

export type CodegenResponse = {
  files: OutputFile[];
  error: undefined;
} | { files: undefined; error: string };
