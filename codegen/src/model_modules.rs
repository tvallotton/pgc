use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use serde::Serialize;

use crate::{
    error::Error,
    r#type::Type,
    request::{Enum, Query, Request, Schema},
    type_builder::{self, TypeBuilder},
};

#[derive(Clone, Serialize)]
pub struct ModelModules {
    type_builder: TypeBuilder,
    pub model_modules: BTreeMap<Rc<str>, ModelModule>,
}

#[derive(Clone, Serialize)]
pub struct ModelModule {
    type_builder: TypeBuilder,
    pub imports: Vec<Rc<str>>,
    pub classes: Vec<ModelClass>,
    pub enums: Rc<[Enum]>,
}

#[derive(Clone, Serialize)]
pub struct ModelClass {
    r#type: Type,
    fields: Vec<(Rc<str>, Type)>,
}

impl ModelModule {
    fn new(type_builder: TypeBuilder) -> Self {
        ModelModule {
            type_builder,
            imports: vec![],
            classes: vec![],
            enums: Default::default(),
        }
    }
}

impl ModelClass {
    fn imports(&self) -> impl Iterator<Item = Rc<str>> + '_ {
        self.fields
            .iter()
            .map(|(_, ty)| ty.import.iter())
            .flatten()
            .cloned()
    }
}

impl ModelModule {
    pub fn imports(&self) -> BTreeSet<Rc<str>> {
        self.classes
            .iter()
            .flat_map(|class| class.imports())
            .collect()
    }
}

impl ModelModules {
    pub fn new(request: &Request) -> Result<Self, Error> {
        let type_builder = TypeBuilder::new(request.clone())?;

        let mut modules = ModelModules {
            type_builder,
            model_modules: Default::default(),
        };

        for schema in request.catalog.schemas.iter() {
            modules.add_schema(schema);
        }

        Ok(modules)
    }

    pub fn add_schema(&mut self, schema: &Schema) {
        let mut module = ModelModule::new(self.type_builder.clone());

        for model in schema.models.iter() {
            let model_class = ModelClass {
                r#type: self.type_builder.resolve(&schema.name, &model.name),
                fields: model
                    .columns
                    .iter()
                    .map(|column| (column.name.clone(), self.type_builder.from_col(column)))
                    .collect::<Vec<_>>(),
            };
            module.classes.push(model_class);
        }

        module.enums = schema.enums.clone();

        self.model_modules.insert(schema.name.clone(), module);
    }
}
