use std::sync::Arc;

use crate::{
    ir::{
        model_modules::{Model, ModelField, ModelModule, ModelModules},
        type_service::TypeService,
    },
    request::{Catalog, Column, Record, Schema},
};

pub struct ModelService {
    pub type_service: TypeService,
    pub catalog: Catalog,
}

impl ModelService {
    pub fn create_model_modules(&self) -> ModelModules {
        let mut modules = ModelModules::default();

        for schema in self.catalog.schemas.iter() {
            let module = self.create_model_module(schema);
            modules.model_modules.insert(schema.name.clone(), module);
        }

        modules
    }

    fn create_model_module(&self, schema: &Schema) -> ModelModule {
        let mut module = ModelModule::new(&schema.name);

        for record in schema.records.iter() {
            let model = self.create_model_from_record(&schema.name, record);
            module.models.push(model);
        }

        module.enums = schema.enums.clone();

        return module;
    }

    fn create_model_from_record(&self, module_name: &Arc<str>, record: &Record) -> Model {
        let mut model = Model {
            record: record.clone(),
            module_name: module_name.clone(),
            name: record.name.clone(),
            fields: vec![],
        };
        for column in record.columns.iter() {
            let field = self.create_model_field_from_column(column);
            model.fields.push(field);
        }
        return model;
    }

    fn create_model_field_from_column(&self, column: &Column) -> ModelField {
        let r#type = self.type_service.from_column(&column);
        ModelField {
            name: column.name.clone(),
            r#type,
            default_value: column.default.clone(),
        }
    }
}
