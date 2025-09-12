use crate::ir::{Ir, IrService};
use crate::request::{Catalog, Column, Config, Query, Request};

use crate::{
    ir::TypeService,
    request::{Enum, Record, Schema},
};
use std::sync::Arc;

#[derive(Default)]
struct RequestBuilder {
    enums: Vec<Enum>,
    records: Vec<Record>,
    queries: Vec<Query>,
}

impl RequestBuilder {
    pub fn add_enum(&mut self, name: &str, values: &[&str]) {
        self.enums.push(Enum {
            name: name.into(),
            values: values
                .into_iter()
                .map(|&str| str.into())
                .collect::<Vec<Arc<str>>>()
                .into(),
        });
    }
}

pub fn enums() -> [Enum; 1] {
    [Enum {
        name: "myenum".into(),
        values: [].into(),
    }]
}

pub fn records() -> [Record; 1] {
    [Record {
        kind: "table".into(),
        name: "".into(),
        columns: Arc::default(),
    }]
}

pub fn schema() -> Schema {
    Schema {
        name: "public".into(),
        enums: enums().into(),
        records: records().into(),
    }
}

pub fn catalog() -> Catalog {
    Catalog {
        schemas: [schema()].into(),
    }
}

pub fn request() -> Request {
    Request {
        catalog: catalog(),
        queries: Default::default(),
        config: Config::default(),
    }
}

pub fn ir() -> Ir {
    IrService::new(request()).unwrap().build()
}
