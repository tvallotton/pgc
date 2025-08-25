use std::sync::Arc;

use super::r#type::Type;
use crate::{
    ir::model_modules::{Model, ModelModules},
    request::{Catalog, Column, OutputType, Record, Request, Schema},
};
#[derive(Clone)]
pub struct TypeService {
    pub catalog: Catalog,
}

impl TypeService {
    pub fn user_defined<'a>(&self, module_path: impl Iterator<Item = &'a str>, name: &str) -> Type {
        Type::UserDefined {
            module_path: module_path.map(|str| str.into()).collect(),
            name: name.into(),
        }
    }

    pub fn resolve_from_output(&self, ty: &OutputType) -> Type {
        self.resolve_from_catalog(&ty.schema, &ty.name)
    }

    pub fn from_column(&self, column: &Column) -> Type {
        let schema_name = &column.type_field.schema_name;
        let column_name = &column.type_field.name;

        let mut r#type = self.resolve_from_catalog(schema_name, column_name);

        if let Some(r#type_) = self.find_table_backed_enum(column) {
            r#type = r#type_;
        }

        if column.type_field.is_array {
            r#type = Type::Array {
                r#type: Arc::new(r#type),
                dim: column.type_field.array_dimensions,
            };
        }

        if column.is_nullable {
            r#type = Type::Nullable(Arc::new(r#type));
        }

        return r#type;
    }

    fn find_table_backed_enum(&self, column: &Column) -> Option<Type> {
        let schema = self.get_schema(column.foreign_table_schema.as_deref()?)?;
        self.resolve_enum(schema, column.foreign_table_name.as_ref()?)
    }

    fn resolve_from_catalog(&self, schema_name: &Arc<str>, name: &Arc<str>) -> Type {
        if &**schema_name == "pg_catalog" {
            return self.from_pg_catalog(&name);
        }
        self.from_user_defined_catalog(schema_name, name)
            .unwrap_or(Type::Any)
    }

    fn from_user_defined_catalog(&self, schema_name: &Arc<str>, name: &Arc<str>) -> Option<Type> {
        let schema = self.get_schema(schema_name)?;

        self.resolve_record(schema, name)
            .or_else(|| self.resolve_enum(schema, name))
    }

    fn resolve_enum(&self, schema: &Schema, name: &Arc<str>) -> Option<Type> {
        schema.enums.iter().find(|enum_| enum_.name == *name)?;
        Some(self.user_defined_model(schema, name))
    }

    fn resolve_record(&self, schema: &Schema, name: &Arc<str>) -> Option<Type> {
        schema.records.iter().find(|record| record.name == *name)?;
        Some(self.user_defined_model(schema, name))
    }

    fn user_defined_model(&self, schema: &Schema, name: &Arc<str>) -> Type {
        let module_path = Arc::new(["models".into(), schema.name.clone()]);
        Type::UserDefined {
            module_path,
            name: name.clone(),
        }
    }

    fn get_schema(&self, schema_name: &str) -> Option<&Schema> {
        self.catalog
            .schemas
            .iter()
            .find(|schema| &*schema.name == schema_name)
    }

    fn from_pg_catalog(&self, type_name: &str) -> Type {
        Type::NAMES
            .iter()
            .find(|(name, _, _)| *name == type_name)
            .map(|(_, _, ty)| ty.clone())
            .unwrap_or(Type::Any)
    }
}
