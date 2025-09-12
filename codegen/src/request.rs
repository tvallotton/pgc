use std::collections::BTreeMap;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub catalog: Catalog,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub queries: Arc<[Query]>,
    pub config: Config,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub schemas: Arc<[Schema]>,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub enums: Arc<[Enum]>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub records: Arc<[Record]>,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcStrFaker"))]
    pub values: Arc<[Arc<str>]>,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub kind: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub columns: Arc<[Column]>,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    #[serde(rename = "type")]
    pub type_field: ColumnType,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub default: Option<Arc<str>>,
    pub is_unique: bool,
    pub is_nullable: bool,
    pub is_foreign_key: bool,
    pub is_primary_key: bool,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub foreign_table_name: Option<Arc<str>>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub foreign_table_schema: Option<Arc<str>>,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnType {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub display: Arc<str>,
    pub is_array: bool,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub schema_name: Arc<str>,
    pub is_composite: bool,
    pub array_dimensions: i64,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub query: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub command: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub path: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub annotations: Arc<BTreeMap<Arc<str>, Annotation>>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub output: Arc<[OutputColumn]>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub parameters: Arc<[Parameter]>,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub value: Option<Arc<str>>,
    pub line: i64,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputColumn {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    #[serde(rename = "type")]
    pub type_: OutputType,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputType {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub schema: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    pub id: i64,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub name: Arc<str>,
    #[serde(rename = "type")]
    pub type_: OutputType,
    pub not_null: bool,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub version: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcStrFaker"))]
    pub queries: Arc<[Arc<str>]>,
    pub codegen: Codegen,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Codegen {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub out: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub language: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub driver: Arc<str>,
    #[serde(default)]
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub types: Arc<BTreeMap<Arc<str>, TypeConfig>>,
    pub options: Value,
}

#[cfg_attr(test, derive(fake::Dummy))]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeConfig {
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcFaker"))]
    pub annotation: Arc<str>,
    #[cfg_attr(test, dummy(faker = "crate::faker::ArcStrFaker"))]
    #[serde(default)]
    pub import: Arc<[Arc<str>]>,
}
