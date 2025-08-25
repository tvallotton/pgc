pub use model::Model;
pub use model::ModelField;
pub use model_module::ModelModule;
use serde::Serialize;
use std::{collections::BTreeMap, sync::Arc};
mod model;
mod model_module;

#[derive(Clone, Serialize, Default)]
pub struct ModelModules {
    pub model_modules: BTreeMap<Arc<str>, ModelModule>,
}
