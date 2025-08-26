use std::collections::BTreeMap;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub catalog: Catalog,
    pub queries: Arc<[Query]>,
    pub config: Config,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    pub schemas: Arc<[Schema]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub name: Arc<str>,
    pub enums: Arc<[Enum]>,
    pub records: Arc<[Record]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    pub name: Arc<str>,
    pub values: Arc<[Arc<str>]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub kind: Arc<str>,
    pub name: Arc<str>,
    pub columns: Arc<[Column]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: Arc<str>,
    #[serde(rename = "type")]
    pub type_field: ColumnType,
    pub default: Option<Arc<str>>,
    pub is_unique: bool,
    pub is_nullable: bool,
    pub is_foreign_key: bool,
    pub is_primary_key: bool,
    pub foreign_table_name: Option<Arc<str>>,
    pub foreign_table_schema: Option<Arc<str>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnType {
    pub name: Arc<str>,
    pub display: Arc<str>,
    pub is_array: bool,
    pub schema_name: Arc<str>,
    pub is_composite: bool,
    pub array_dimensions: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub query: Arc<str>,
    pub name: Arc<str>,
    pub command: Arc<str>,
    pub path: Arc<str>,
    pub annotations: Arc<BTreeMap<String, Annotation>>,
    pub output: Arc<[OutputColumn]>,
    pub parameters: Arc<[Parameter]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    pub value: Option<Arc<str>>,
    pub line: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputColumn {
    pub name: Arc<str>,
    #[serde(rename = "type")]
    pub type_: OutputType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputType {
    pub schema: Arc<str>,
    pub name: Arc<str>,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Arc<str>,
    #[serde(rename = "type")]
    pub type_: OutputType,
    pub not_null: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub version: Arc<str>,
    pub queries: Arc<[Arc<str>]>,
    pub codegen: Codegen,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Codegen {
    pub out: Arc<str>,
    pub language: Arc<str>,
    pub driver: Arc<str>,
    #[serde(default)]
    pub types: Arc<BTreeMap<Arc<str>, TypeConfig>>,
    pub options: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeConfig {
    pub annotation: Arc<str>,
    #[serde(default)]
    pub import: Vec<Arc<str>>,
}
