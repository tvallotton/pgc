import { ConfigService } from "../config/config.service.ts";
import { Catalog } from "./schema.types.ts";

export class ExcluderService {
  constructor(readonly configService: ConfigService) {}

  removeExcludedModels(catalog: Catalog) {
    const exclude = this.configService.config.codegen.exclude_models ?? [];
    for (const modelName of exclude) {
      const [name, schemaName] = modelName.split(".").reverse();

      const schema = catalog.schemas.find((schema) =>
        schema.name == (schemaName ?? "public")
      );

      if (!schema) return;
      schema.models = schema.models.filter((table) => table.name != name);
    }
  }
}
