use std::{collections::BTreeMap, sync::Arc, sync::LazyLock};

use serde::Deserialize;
use serde_json::json;

use crate::{
    error::Error,
    request::{self, Column, OutputType, TypeConfig},
    utils::render,
};

#[derive(serde::Serialize, Deserialize, Clone, Debug)]
pub struct Type {
    #[serde(default)]
    pub declaration: Arc<str>,
    #[serde(default)]
    pub annotation: Arc<str>,
    #[serde(default)]
    pub constructor: Arc<str>,
    #[serde(default)]
    pub import: Arc<[Arc<str>]>,
    pub pgtype_name: Option<Arc<str>>,
    pub pgtype_schema: Option<Arc<str>>,
}
