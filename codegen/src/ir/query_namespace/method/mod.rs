use std::{collections::BTreeMap, sync::Arc};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{ir::r#type::Type, request::Query};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Method {
    pub query: Query,
    pub arguments: IndexMap<Arc<str>, Type>,
    pub input_models: BTreeMap<Arc<str>, MethodModel>,
    pub output_type: Option<Type>,
    pub output_model: Option<MethodModel>,
    pub output_columns: IndexMap<Arc<str>, Type>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MethodModel {
    pub r#type: Type,
    pub fields: IndexMap<Arc<str>, Type>,
}

impl Method {
    pub fn used_types(&self) -> impl Iterator<Item = Type> + '_ {
        let argument_imports = self.arguments.values().cloned();
        self.input_models
            .values()
            .chain(self.output_model.as_ref())
            .flat_map(|model| model.fields.iter())
            .map(|field| field.1.clone())
            .chain(argument_imports)
    }
}
