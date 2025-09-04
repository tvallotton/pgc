use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
mod method;
use crate::{
    error::Error,
    ir::{query_namespace_service::QueryNamespaceService, r#type::Type},
    request::Request,
};
pub use method::Method;
pub use method::MethodModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryNamespace {
    pub name: String,
    pub subnamespaces: BTreeMap<Arc<str>, QueryNamespace>,
    pub methods: Vec<Method>,
}

impl QueryNamespace {
    pub fn from_request(request: &Request) -> Result<Self, Error> {
        Ok(QueryNamespaceService::new(request)?.build())
    }

    pub fn root() -> QueryNamespace {
        QueryNamespace {
            name: String::new(),
            subnamespaces: Default::default(),
            methods: Default::default(),
        }
    }

    pub fn used_types(&self) -> BTreeSet<Type> {
        self.methods
            .iter()
            .flat_map(|method| method.used_types())
            .collect()
    }

    pub fn resolve(&mut self, name: &str) -> &mut QueryNamespace {
        self._resolve(&name.split('.').collect::<Vec<_>>())
    }

    pub fn _resolve(&mut self, name: &[&str]) -> &mut QueryNamespace {
        if name.is_empty() {
            return self;
        }

        let entry = self.subnamespaces.entry(name[0].into());

        let namespace = entry.or_insert_with(|| QueryNamespace {
            name: name[0].into(),
            methods: Default::default(),
            subnamespaces: Default::default(),
        });

        return namespace._resolve(&name[1..]);
    }
}
