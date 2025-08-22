import type { PGService } from "../pg/pg.service.ts";
import type { Annotation, Parameter, Query, RawQuery } from "./query.types.ts";

const ANNOTATION = /\s*--\s*@(\w+)(?::\s*([^\n]+))?\s*/;
const PARAMETER =
  /([?\$][A-Za-z]\w*|\$\([A-Za-z]\w*\))|[?\$]\(([A-Za-z]\w*).([A-Za-z]\w*)\)/g;
const COMMENT = /\s*--[^\n]+\n/g;

export class QueryParserService {
  constructor(
    readonly pgService: PGService,
  ) {}

  async parseQuery(rawQuery: RawQuery): Promise<Query | undefined> {
    const annotations = this.parseAnnotations(rawQuery);

    if (!annotations["name"]) return;
    const { name, command } = this.parseName(rawQuery, annotations)!;
    const { query, params } = this.replaceParameters(rawQuery.sql);
    const { inputs, outputs } = await this.pgService.describe({
      ...rawQuery,
      sql: query,
    });

    return {
      query,
      name,
      command,
      path: rawQuery.file.path,
      annotations,
      output: outputs,
      parameters: params.entries().map(([name, nullableness], i) => ({
        name: name,
        not_null: nullableness == Nullableness.NON_NULL,
        type: inputs[i],
      })).filter(({ type }) => type).toArray(),
    };
  }

  parseAnnotations(query: RawQuery): Record<string, Annotation> {
    let lines = -1;
    const annotations: Record<string, Annotation> = {};
    for (const line of query.sql.split("\n")) {
      lines++;
      const match = line.match(ANNOTATION);
      if (!match) continue;
      annotations[match[1]] = {
        value: match[2] ?? "",
        line: lines + query.line,
      };
    }
    return annotations;
  }

  parseName(query: RawQuery, annotations: Record<string, Annotation>) {
    const name = annotations["name"];
    const match = name.value.match(
      /(\S+)\s+:(val|exec|one|many)/,
    );
    if (!match) {
      throw Error(
        `"${query.file.path}:${name.line}" invalid query return specifier (expected one of: :val, :one, :many, :exec)`,
      );
    }

    query.line = name.line;

    return {
      name: match[1],
      command: match[2],
    };
  }

  replaceParameters(rawQuery: string) {
    const params = new Map<string, Nullableness>();

    const matches = rawQuery.matchAll(PARAMETER);
    let query = rawQuery;

    for (const [full, g1, g2, g3] of matches) {
      const paramType = full.startsWith("$")
        ? Nullableness.NON_NULL
        : Nullableness.NULLABLE;
      const param = g3 != undefined ? `${g2}.${g3}` : g1;
      if (!params.has(param)) {
        params.set(
          param.replaceAll(/[\($?\)]/g, ""),
          paramType,
        );

        query = query.replaceAll(full, `$${params.size}`);
        query = query.replaceAll(full, `$${params.size}`);
      }
    }

    query = query.replaceAll(COMMENT, "").trim();

    return { rawQuery, query, params };
  }
}

enum Nullableness {
  NULLABLE,
  NON_NULL,
}
