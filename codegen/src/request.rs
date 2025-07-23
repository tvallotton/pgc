use std::collections::BTreeMap;
use std::rc::Rc;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub catalog: Catalog,
    pub queries: Rc<[Query]>,
    pub config: Config,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    pub schemas: Rc<[Schema]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub name: Rc<str>,
    pub enums: Rc<[Enum]>,
    pub models: Rc<[Model]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    pub name: Rc<str>,
    pub values: Rc<[Rc<str>]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    pub kind: Rc<str>,
    pub name: Rc<str>,
    pub columns: Rc<[Column]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: Rc<str>,
    #[serde(rename = "type")]
    pub type_field: Type,
    pub default: Option<Rc<str>>,
    pub is_unique: bool,
    pub is_nullable: bool,
    pub is_foreign_key: bool,
    pub is_primary_key: bool,
    pub foreign_table_name: Option<Rc<str>>,
    pub foreign_table_schema: Option<Rc<str>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub name: Rc<str>,
    pub display: Rc<str>,
    pub is_array: bool,
    pub schema_name: Rc<str>,
    pub is_composite: bool,
    pub array_dimensions: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub query: Rc<str>,
    pub name: Rc<str>,
    pub command: Rc<str>,
    pub path: Rc<str>,
    pub annotations: Annotations,
    pub output: Rc<[OutputColumn]>,
    pub parameters: Rc<[Parameter]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotations {
    pub name: Name,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub value: Rc<str>,
    pub line: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputColumn {
    pub name: Rc<str>,
    #[serde(rename = "type")]
    pub type_: QueryType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryType {
    pub schema: Rc<str>,
    pub name: Rc<str>,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Rc<str>,
    #[serde(rename = "type")]
    pub type_: QueryType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub version: Rc<str>,
    pub queries: Rc<[Rc<str>]>,
    pub disable_cache: bool,
    pub database: Database,
    pub codegen: Codegen,
    pub env_file: Rc<[Rc<str>]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub migrations: Rc<str>,
    pub pglite: Pglite,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pglite {
    pub extensions: Extensions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extensions {
    pub vector: Rc<str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Codegen {
    pub out: Rc<str>,
    pub target: Rc<str>,
    #[serde(default)]
    pub types: BTreeMap<Rc<str>, TypeConfig>,
    pub options: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeConfig {
    name: Rc<str>,
    annotation: Rc<str>,
    import: Rc<[Rc<str>]>,
}
