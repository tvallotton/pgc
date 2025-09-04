use std::{collections::BTreeSet, sync::Arc};

use serde::Serialize;

use crate::{
    ir::{model_modules::Model, Type},
    request::Enum,
};

#[derive(Clone, Serialize)]
pub struct ModelModule {
    pub name: Arc<str>,
    pub models: Vec<Model>,
    pub enums: Arc<[Enum]>,
}

impl ModelModule {
    pub fn new(name: &Arc<str>) -> Self {
        ModelModule {
            name: name.clone(),
            models: vec![],
            enums: Default::default(),
        }
    }

    pub fn used_types(&self) -> BTreeSet<Type> {
        self.models
            .iter()
            .flat_map(|field| field.fields.iter())
            .map(|ty| ty.r#type.clone())
            .collect()
    }
}
