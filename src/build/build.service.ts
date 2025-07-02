import type { ConfigService } from "../config/config.service.ts";
import { FileCollectorService } from "../fs/file_collector.service.ts";
import { PGService } from "../pg/pg.service.ts";
import { RawQueryCollector } from "../query_collector/query_collector.servce.ts";
import { QueryParserService } from "../query_collector/query_parser.service.ts";
import { SchemaService } from "../schema_service/schema.service.ts";

export class BuildService {
  constructor(
    readonly configService: ConfigService,
    readonly pgService: PGService,
    readonly fileCollectorService: FileCollectorService,
    readonly rawQueryCollector: RawQueryCollector,
    readonly queryParser: QueryParserService,
    readonly schemaService: SchemaService,
  ) {}

  async static(configService: ConfigService) {
    const pgService = await PGService.fromConfig(configService);

    const fileCollectorService = new FileCollectorService(configService);

    const rawQueryCollector = new RawQueryCollector(fileCollectorService);

    const queryParser = new QueryParserService(pgService);
    const schemaService = new SchemaService(pgService, fileCollectorService);

    return new BuildService(
      configService,
      pgService,
      fileCollectorService,
      rawQueryCollector,
      queryParser,
      schemaService,
    );
  }

  async build() {
  }

  async getCatalog() {
    return await this.schemaService.loadCatalog();
  }

  async getQueries() {
    const rawQueries = await this.rawQueryCollector.loadQueries();
    const queries = [];
    for (const rawQuery of rawQueries) {
      queries.push(
        await this.queryParser.parseQuery(rawQuery),
      );
    }
    return queries;
  }
}
