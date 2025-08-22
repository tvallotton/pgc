import { PGlite } from "@electric-sql/pglite";
const pg = new PGlite();
type Step = (cell: string) => any;

function trimOuter(str: string, open: string, close: string) {
  const s = str.trim();
  if (s.startsWith(open) && s.endsWith(close)) return s.slice(1, -1);
  return s;
}

function unquote(s: string): string {
  const t = s.trim();
  if (t.length >= 2 && t.startsWith('"') && t.endsWith('"')) {
    // Remove surrounding quotes and unescape \" and \\ (good enough for most PG cases)
    return t
      .slice(1, -1)
      .replace(/\\(["\\])/g, "$1");
  }
  return t;
}

function splitTopLevel(
  s: string,
  separator: string,
  { respectQuotes = true, parens = true, braces = true }: {
    respectQuotes?: boolean;
    parens?: boolean;
    braces?: boolean;
  } = {},
): string[] {
  const out: string[] = [];
  let buf = "";
  let inQuotes = false;
  let parenDepth = 0;
  let braceDepth = 0;

  const flush = () => {
    out.push(buf);
    buf = "";
  };

  for (let i = 0; i < s.length; i++) {
    const ch = s[i];

    if (respectQuotes && ch === '"' && s[i - 1] !== "\\") {
      inQuotes = !inQuotes;
      buf += ch;
      continue;
    }
    if (!inQuotes) {
      if (parens && (ch === "(" || ch === ")")) {
        if (ch === "(") parenDepth++;
        else parenDepth--;
        buf += ch;
        continue;
      }
      if (braces && (ch === "{" || ch === "}")) {
        if (ch === "{") braceDepth++;
        else braceDepth--;
        buf += ch;
        continue;
      }
      if (parenDepth === 0 && braceDepth === 0 && ch === separator) {
        flush();
        continue;
      }
    }
    buf += ch;
  }
  flush();
  return out.map((t) => t.trim());
}

function parsePgRowToCells(row: string): string[] {
  const inner = trimOuter(row.trim(), "(", ")");
  if (inner === "") return [];
  // Note: allow parentheses/braces in cells; split only at top-level commas
  return splitTopLevel(inner, ",");
}

function parsePgArrayToElements(arr: string): string[] {
  const inner = trimOuter(arr.trim(), "{", "}");
  if (inner === "") return [];
  // In arrays, elements can be quoted (including quoted rows "(...)")
  return splitTopLevel(inner, ",", {
    respectQuotes: true,
    parens: true,
    braces: true,
  });
}

// ---- Scalar parsers ----
function parseNumber(cell: string): number {
  const t = cell.trim();
  if (t.toUpperCase() === "NULL" || t === "") return NaN; // choose your null policy
  const q = unquote(t);
  const v = Number(q);
  if (Number.isNaN(v)) throw new Error(`Invalid number: ${cell}`);
  return v;
}

function parseString(cell: string): string | null {
  const t = cell.trim();
  if (cell == "") return null as any;
  return unquote(t).replaceAll(/""/g, '"');
}

function parseDate(cell: string): Date {
  const t = unquote(cell.trim());
  const d = new Date(t);
  if (Number.isNaN(d.getTime())) throw new Error(`Invalid date: ${cell}`);
  return d;
}

function parseBoolean(cell: string): boolean {
  const t = unquote(cell.trim());
  if (!["t", "f"].includes(t)) {
    throw new Error(`Invalid boolean: ${cell}. Expected "t" or "f".`);
  }
  return t == "t";
}

class ArrayParser<T> {
  constructor(readonly map: (_: string) => T) {}
  parse(array: string): T[] {
    const unquoted = unquote(array.trim());
    return parsePgArrayToElements(unquoted).map(this.map);
  }

  arrayOfThis() {
    return new ArrayParser((e) => this.parse(e));
  }
}

export class RowParser<T extends any[] = [], V = T> {
  private steps: Step[];
  private mapFun: (_: T) => V;
  constructor(steps: Step[] = [], map?: (_: T) => V) {
    this.steps = steps;
    this.mapFun = map ?? ((row: T) => row as unknown as V);
  }

  number(): RowParser<[...T, number]> {
    return new RowParser<[...T, number]>([...this.steps, parseNumber]);
  }

  string(): RowParser<[...T, string]> {
    return new RowParser<[...T, string]>([...this.steps, parseString]);
  }

  date(): RowParser<[...T, Date]> {
    return new RowParser<[...T, Date]>([...this.steps, parseDate]);
  }

  boolean(): RowParser<[...T, boolean]> {
    return new RowParser<[...T, boolean]>([...this.steps, parseBoolean]);
  }

  row<U extends any[]>(sub: RowParser<U>): RowParser<[...T, U]> {
    const step: Step = (cell: string) => {
      const raw = unquote(cell.trim()); // nested rows are often quoted inside rows/arrays
      return sub.parse(raw);
    };
    return new RowParser<[...T, U]>([...this.steps, step]);
  }

  arrayOfNumber(): RowParser<[...T, number[]]> {
    const step: Step = (cell: string) => {
      return new ArrayParser(parseNumber).parse(cell);
    };
    return new RowParser<[...T, number[]]>([...this.steps, step]);
  }

  arrayOfDate(): RowParser<[...T, Date[]]> {
    const step: Step = (cell: string) => {
      return new ArrayParser(parseDate).parse(cell);
    };
    return new RowParser<[...T, Date[]]>([...this.steps, step]);
  }

  arrayOfRow<U>(sub: RowParser<any, U>): RowParser<[...T, U]> {
    const step: Step = (cell: string) => {
      // Each element is typically a quoted row string "(...)"
      return sub.arrayOfThis().parse(cell);
    };
    return new RowParser<[...T, U]>([...this.steps, step]);
  }

  arrayOfThis(): ArrayParser<V> {
    return new ArrayParser((e) => this.parse(unquote(e)));
  }

  parse(input: string): V {
    const trimmed = input.trim();
    // Accept either full row "(a,b,...)" or a bare CSV (we’ll try row first)
    const cells = trimmed.startsWith("(")
      ? parsePgRowToCells(trimmed)
      : splitTopLevel(trimmed, ",");
    if (cells.length !== this.steps.length) {
      throw new Error(
        `Arity mismatch: expected ${this.steps.length} fields, got ${cells.length} (${
          JSON.stringify(cells)
        })`,
      );
    }
    const out = this.steps.map((fn, i) => fn(cells[i])) as T;
    return this.mapFun(out);
  }

  map<U>(fun: (_: V) => U): RowParser<T, U> {
    const newMap = (row: T) => fun(this.mapFun(row));
    return new RowParser<T, U>(this.steps, newMap);
  }
}

const authorParser = new RowParser()
  .number()
  .string()
  .string()
  .map(([id, firstName, lastName]) => ({
    id,
    firstName,
    lastName,
  }));

const parser = {
  author: new RowParser()
    .number()
    .string()
    .string()
    .map(([id, firstName, lastName]) => ({
      id,
      firstName,
      lastName,
    })),
};

const { rows } = await pg.query(
  `select array[row(true, 1, null, 'asd""')] as row`,
);

authorParser.arrayOfThis().parse(rows[0].row);
