use std::rc::Rc;

use serde::Serialize;

use crate::{ir::model_modules::Model, request::Enum};

#[derive(Clone, Serialize)]
pub struct ModelModule {
    pub name: Rc<str>,
    pub models: Vec<Model>,
    pub enums: Rc<[Enum]>,
}

impl ModelModule {
    pub fn new(name: &Rc<str>) -> Self {
        ModelModule {
            name: name.clone(),
            models: vec![],
            enums: Default::default(),
        }
    }
}
