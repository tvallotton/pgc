use std::{
    collections::{btree_map::Entry, BTreeMap},
    rc::Rc,
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{r#type::Type, request::Query};

pub use method_builder::MethodBuilder;
mod method_builder;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Method {
    query: Query,
    arguments: IndexMap<Rc<str>, Type>,
    input_models: BTreeMap<Rc<str>, MethodModel>,
    pub output_type: Option<Type>,
    output_model: Option<MethodModel>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MethodModel {
    r#type: Type,
    fields: IndexMap<Rc<str>, Type>,
}

impl Method {
    pub fn imports(&self) -> impl Iterator<Item = &str> + '_ {
        let argument_imports = self.arguments.values().flat_map(|ty| ty.import.iter());
        self.input_models
            .values()
            .chain(self.output_model.as_ref())
            .flat_map(|model| model.fields.iter())
            .flat_map(|field| field.1.import.iter())
            .chain(argument_imports)
            .map(|v| &**v)
    }
}
