use crate::{
    error::Error,
    ir::{model_service::ModelService, query_namespace_service::QueryNamespaceService},
    request::{Catalog, Request},
};
pub use model_modules::*;
pub use query_namespace::*;
use serde::Serialize;
pub use r#type::Type;
pub use type_service::TypeService;
mod method_service;
mod model_modules;
mod model_service;
mod query_namespace;
mod query_namespace_service;
mod r#type;
mod type_service;

#[derive(Serialize, Clone)]
pub struct Ir {
    pub request: Request,
    pub query_namespace: query_namespace::QueryNamespace,
    pub model_modules: model_modules::ModelModules,
}

pub struct IrService {
    query_namespace_service: QueryNamespaceService,
    model_service: ModelService,
}

impl IrService {
    pub fn new(request: Request) -> Result<Self, Error> {
        let type_service = TypeService {
            catalog: request.catalog.clone(),
        };
        let query_namespace_service = QueryNamespaceService::new(&request)?;
        let model_service = ModelService {
            type_service,
            catalog: request.catalog.clone(),
        };
        Ok(IrService {
            query_namespace_service,
            model_service,
        })
    }

    pub fn build(&mut self, request: Request) -> Ir {
        let model_modules = self.model_service.create_model_modules();
        let query_namespace = self.query_namespace_service.build();
        Ir {
            model_modules,
            query_namespace,
            request,
        }
    }
}
