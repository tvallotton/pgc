use crate::{
    query_namespace::QueryNamespace,
    request::{Model, Query},
};

pub struct TemplateContext {
    namespaces: QueryNamespace,
}
