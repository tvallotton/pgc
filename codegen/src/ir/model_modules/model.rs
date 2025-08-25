use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    ir::r#type::Type,
    request::{Column, Record},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub record: Record,
    pub module_name: Arc<str>,
    pub name: Arc<str>,
    pub fields: Vec<ModelField>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModelField {
    pub name: Arc<str>,
    pub r#type: Type,
    pub default_value: Option<Arc<str>>,
}
