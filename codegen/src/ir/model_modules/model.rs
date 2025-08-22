use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{
    ir::r#type::Type,
    request::{Column, Record},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub record: Record,
    pub module_name: Rc<str>,
    pub name: Rc<str>,
    pub fields: Vec<ModelField>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModelField {
    pub name: Rc<str>,
    pub r#type: Type,
    pub default_value: Option<Rc<str>>,
}
