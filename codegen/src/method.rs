use crate::request::{Model, Query};

pub struct Method {
    name: String,
    command: String,
    query: Query,
    output: Model,
}
