use crate::{error::Error, request::Request};
pub use model_modules::*;
pub use query_namespace::*;
pub use r#type::Type;
use serde::Serialize;
pub use type_service::TypeService;
mod method_service;
mod model_modules;
mod model_service;
mod query_namespace;
mod query_namespace_service;
mod r#type;
mod type_service;

#[derive(Serialize)]
pub struct Ir {
    pub request: Request,
    pub query_namespace: query_namespace::QueryNamespace,
    pub model_modules: model_modules::ModelModules,
}
