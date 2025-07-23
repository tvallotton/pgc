use std::{any::type_name, collections::BTreeMap, rc::Rc, sync::LazyLock};

use serde::Deserialize;

use crate::request::{self, TypeConfig};

pub struct Type {
    declaration: Rc<str>,
    annotation: Rc<str>,
    constructor: Rc<str>,
    pgtype_name: Rc<str>,
    pgtype_schema: Rc<str>,
}

pub struct TypeConstructor<'a>(pub &'a request::Request);

impl<'a> TypeConstructor<'a> {
    fn _from_pg(
        &self,
        type_schema: Rc<str>,
        type_name: Rc<str>,
    ) -> Result<Option<TypeConfig>, String> {
        let name = format!("{}.{}", type_schema, type_name);

        if let Some(value) = self.0.config.codegen.types.get(&*name) {
            return Ok(Some(value.clone()));
        }

        return self.try_predefined(type_schema, type_name);
    }

    fn try_predefined(
        &self,
        type_schema: Rc<str>,
        type_name: Rc<str>,
    ) -> Result<Option<TypeConfig>, String> {
        match &*self.0.config.codegen.target {
            "python" | "py" | "python3" => Ok(PYTHON_TYPES
                .get(&*type_schema)
                .map(|schema| schema.get(&*type_name))
                .flatten()
                .cloned()),
            lang => Err(format!("language {lang} is not supported.")),
        }
    }
}

const PYTHON_TYPES: LazyLock<BTreeMap<Rc<str>, BTreeMap<Rc<str>, TypeConfig>>> =
    LazyLock::new(|| {
        let json = include_str!("../templates/python/types.json");
        return serde_json::from_str(json).unwrap();
    });
