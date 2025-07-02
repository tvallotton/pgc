interface Schema {}

interface Catalog {
  name: string;
  defaultSchema: string;
  schemas: Schema[];
}
interface Schema {
  name: string;
  tables: Table;
}

interface Table {
  catalog: string;
  schema: string;
  name: string;
  columns: Column[];
  type: TableType;
  isInsertableInto: boolean;
}

enum TableType {
  BASE_TABLE = "BASE TABLE",
  VIEW = "VIEW",
  FOREIGN = "FOREIGN",
  LOCAL_TEMPORARY = "LOCAL TEMPORARY",
}

interface Column {
  catalog: string;
  schema: string;
  table: string;
  name: string;
  ordinalPosition: number;
  default: string | null;
  is_nullable: boolean;
  type: string;
  characterMaximumLength: number | null;
  characterOctetLength;
}
