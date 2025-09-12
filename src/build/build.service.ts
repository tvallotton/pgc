import { CodegenService } from "../codegen/codegen.service.ts";
import { ConfigService } from "../config/config.service.ts";
import { FileCollectorService } from "../fs/file_collector.service.ts";
import { PGService } from "../pg/pg.service.ts";
import { RawQueryCollector } from "../query_collector/query_collector.servce.ts";
import { QueryParserService } from "../query_collector/query_parser.service.ts";
import { ExcluderService } from "../schema_service/excluder.service.ts";
import { SchemaService } from "../schema_service/schema.service.ts";

export class BuildService {
  constructor(
    readonly configService: ConfigService,
    readonly pgService: PGService,
    readonly fileCollectorService: FileCollectorService,
    readonly rawQueryCollector: RawQueryCollector,
    readonly queryParser: QueryParserService,
    readonly schemaService: SchemaService,
    readonly codegenService: CodegenService,
  ) {}

  static async fromConfig(configService: ConfigService) {
    const pgService = await PGService.fromConfig(configService);

    const fileCollectorService = new FileCollectorService(configService);

    const rawQueryCollector = new RawQueryCollector(fileCollectorService);

    const queryParser = new QueryParserService(pgService);
    const excludedService = new ExcluderService(configService);
    const schemaService = new SchemaService(
      pgService,
      fileCollectorService,
      configService,
      excludedService,
    );
    const codegenService = new CodegenService(configService);

    return new BuildService(
      configService,
      pgService,
      fileCollectorService,
      rawQueryCollector,
      queryParser,
      schemaService,
      codegenService,
    );
  }

  async build() {
    const payload = {
      catalog: await this.getCatalog(),
      queries: await this.getQueries(),
      config: this.configService.config,
    };

    await this.codegenService.generate(payload);
  }

  async getCatalog() {
    return await this.schemaService.loadCatalog();
  }

  async getQueries() {
    const rawQueries = await this.rawQueryCollector.loadQueries();
    const queries = [];
    for (const rawQuery of rawQueries) {
      const query = await this.queryParser.parseQuery(rawQuery);
      if (query) {
        queries.push(query);
      }
    }
    return queries;
  }

  close() {
    return this.pgService.close();
  }
}
