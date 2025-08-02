use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use crate::{error::Error, method::Method, request::Request, utils::to_pascal_case};
pub use query_namespace_builder::QueryNamespaceBuilder;
use serde::{Deserialize, Serialize};
mod query_namespace_builder;

#[derive(Serialize, Deserialize)]
pub struct QueryNamespace {
    pub name: String,
    pub subnamespaces: BTreeMap<Rc<str>, QueryNamespace>,
    pub methods: Vec<Method>,
}

impl QueryNamespace {
    pub fn from_request(request: &Request) -> Result<Self, Error> {
        Ok(QueryNamespaceBuilder::new(request)?.build())
    }

    fn root() -> QueryNamespace {
        QueryNamespace {
            name: String::new(),
            subnamespaces: Default::default(),
            methods: Default::default(),
        }
    }

    pub fn imports(&self) -> BTreeSet<&str> {
        self.methods
            .iter()
            .flat_map(|method| method.imports())
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
