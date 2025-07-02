import type { File } from "../fs/fs.types.ts";
import type { PGType } from "../pg/pg.types.ts";

export interface SQLType {
  schema: string;
  name: string;
  id: number;
}

export interface Parameter {
  name: string;
  type: PGType;
}

interface Column {
  name: string;
  type: PGType;
}

enum Command {
  EXEC,
  MANY,
  ONE,
}

export interface Annotation {
  value: string;
  line: number;
}

export interface Query {
  command: string;
  name: string;
  annotations: Record<string, Annotation>;
  query: string;
  parameters: Parameter[];
  path: string;
  output: Column[];
}

export interface RawQuery {
  sql: string;
  line: number;
  file: File;
}
