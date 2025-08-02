use std::{collections::BTreeMap, rc::Rc, sync::LazyLock};

use serde::Deserialize;
use serde_json::json;

use crate::{
    error::Error,
    request::{self, Column, QueryType, TypeConfig},
    utils::render,
};

#[derive(serde::Serialize, Deserialize, Clone, Debug)]
pub struct Type {
    #[serde(default)]
    pub declaration: Rc<str>,
    #[serde(default)]
    pub annotation: Rc<str>,
    #[serde(default)]
    pub constructor: Rc<str>,
    #[serde(default)]
    pub import: Rc<[Rc<str>]>,
    pub pgtype_name: Option<Rc<str>>,
    pub pgtype_schema: Option<Rc<str>>,
}
