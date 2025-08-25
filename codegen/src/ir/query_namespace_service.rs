use crate::{
    error::Error,
    ir::{
        method_service::MethodService,
        query_namespace::QueryNamespace,
        type_service::{self, TypeService},
    },
    request::{Query, Request},
};

pub struct QueryNamespaceBuilder {
    request: Request,
    method_service: MethodService,
    namespace: QueryNamespace,
}

impl QueryNamespaceBuilder {
    pub fn new(request: &Request) -> Result<QueryNamespaceBuilder, Error> {
        let type_service = TypeService {
            catalog: request.catalog.clone(),
        };
        Ok(QueryNamespaceBuilder {
            request: request.clone(),
            method_service: MethodService::new(type_service.clone()),
            namespace: QueryNamespace::root(),
        })
    }

    pub fn build(&mut self) -> QueryNamespace {
        for query in self.request.queries.clone().iter() {
            self.include_query(query);
        }

        std::mem::replace(&mut self.namespace, QueryNamespace::root())
    }

    pub fn include_query(&mut self, query: &Query) {
        let name = query.namespace();
        let namespace = self.namespace.resolve(name);
        namespace.methods.push(self.method_service.build(query));
    }
}

impl Query {
    pub fn namespace(&self) -> &str {
        if let Some(namespace) = self
            .annotations
            .get("namespace")
            .and_then(|n| n.value.as_deref())
        {
            return &*namespace;
        }

        return self
            .path
            .split('/')
            .last()
            .unwrap()
            .trim_end_matches(".sql");
    }
}
