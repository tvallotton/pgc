use crate::{
    error::Error,
    method::MethodBuilder,
    query_namespace::QueryNamespace,
    request::{Query, Request},
    type_builder::TypeBuilder,
};

pub struct QueryNamespaceBuilder {
    request: Request,
    method_builder: MethodBuilder,
    namespace: QueryNamespace,
}

impl QueryNamespaceBuilder {
    pub fn new(request: &Request) -> Result<QueryNamespaceBuilder, Error> {
        let type_builder = TypeBuilder::new(request.clone())?;
        Ok(QueryNamespaceBuilder {
            request: request.clone(),
            method_builder: MethodBuilder::new(type_builder.clone()),
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
        namespace.methods.push(self.method_builder.build(query));
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
