use crate::{error::Error, request::Request};

mod method_service;
mod model_modules;
mod model_service;
mod query_namespace;
mod query_namespace_service;
mod r#type;
mod type_service;

pub struct Ir {
    request: Request,
    query_namespace: query_namespace::QueryNamespace,
    model_modules: model_modules::ModelModules,
}
