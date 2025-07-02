import type { FileCollectorService } from "../fs/file_collector.service.ts";
import type { File } from "../fs/fs.types.ts";
import type { Query, RawQuery } from "./query.types.ts";

export class RawQueryCollector {
  queries: RawQuery[];
  constructor(readonly fileCollectorService: FileCollectorService) {
    this.queries = [];
  }

  async loadQueries(): Promise<RawQuery[]> {
    const files = await this.fileCollectorService.getQueryFiles();

    for (const file of files) {
      this.parseQueries(await file);
    }

    return this.queries;
  }

  parseQueries(file: File) {
    this.queries = [
      ...this.queries,
      ...this.splitQueries(file).filter((query) => query.sql.trim() != ""),
    ];
  }

  parseComments(query: string) {
    return [...query.matchAll(/\s*--[\n]+\n/)];
  }

  splitQueries(file: File) {
    const queries: RawQuery[] = [];
    let startIdx = 0;
    let index = 0;
    let startLine = 1;
    let line = 1;
    let ignoreSemicolonUntil: string | undefined = undefined;
    for (const char of file.content) {
      index += 1;

      if (char == "\n") {
        line++;
      }

      if (char == ignoreSemicolonUntil) {
        ignoreSemicolonUntil = undefined;
        continue;
      }

      if (['"', "'"].includes(char)) {
        ignoreSemicolonUntil = char;
      }

      if (char == "-" && file.content[index] == "-") {
        ignoreSemicolonUntil = "\n";
      }

      if (ignoreSemicolonUntil == undefined && char == ";") {
        queries.push({
          file,
          line: startLine,
          sql: file.content.slice(startIdx, index),
        });
        startIdx = index;
        startLine = line;
      }
    }

    queries.push({
      file,
      line: startLine,
      sql: file.content.slice(startIdx, index),
    });

    return queries;
  }
}
