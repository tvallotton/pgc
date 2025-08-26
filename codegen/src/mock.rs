use crate::request::{Catalog, Request};

use crate::{
    ir::TypeService,
    request::{Enum, Record, Schema},
};
use std::sync::Arc;

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
