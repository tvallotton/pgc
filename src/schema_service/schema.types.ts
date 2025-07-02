export interface Catalog {
  schemas: Schema[];
}

export interface Schema {
  name: string;
  tables: Table;
}

export interface Table {
  kind: "table" | "view" | "materialized view" | "composite";
  name: string;
  columns: Column[];
}

export interface Column {
  name: string;
  type: SQLType;
  default: string | null;
  is_unique: boolean;
  is_nullable: boolean;
  is_foreign_key: boolean;
  is_primary_key: boolean;
  foreign_table_name: string | null;
  foreign_table_schema: string | null;
}

export interface SQLType {
  name: string;
  display: string;
  is_array: boolean;
  schema_name: string;
  is_composite: boolean;
  array_dimensions: number;
}
