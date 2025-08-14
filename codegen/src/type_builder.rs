use std::{collections::BTreeMap, rc::Rc, sync::LazyLock};

use minijinja::context;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    r#type::Type,
    request::{Catalog, Column, ColumnType, OutputType, Request, TypeConfig},
    utils::render,
};

/// It creates instances of `Type`, either of newly declared types
/// or by transforming an SQL type into a `Type`.
#[derive(Deserialize, Serialize, Clone)]
pub struct TypeBuilder {
    type_overrides: Rc<BTreeMap<Rc<str>, TypeConfig>>,
    enums: Vec<(Rc<str>, Rc<str>)>,
    catalog: Catalog,
    type_map: TypeMap,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TypeMap {
    new_type_case: String,
    null: Type,
    array: Type,
    composite: Type,
    wildcard: TypeConfig,
    schema: BTreeMap<String, BTreeMap<String, TypeConfig>>,
}

impl TypeBuilder {
    pub fn new(request: Request) -> Result<Self, Error> {
        let type_overrides = request.config.codegen.types.clone();
        let lang = request.config.codegen.target.clone();
        let enums: Vec<_> = request
            .catalog
            .schemas
            .iter()
            .flat_map(|schema| {
                schema
                    .enums
                    .iter()
                    .map(move |enum_| (schema.name.clone(), enum_.name.clone()))
            })
            .collect();
        let resolver = match &*lang {
            "python:asyncpg" => TypeBuilder {
                type_overrides,
                enums,
                catalog: request.catalog.clone(),
                type_map: PYTHON_ASYNCPG.clone(),
            },
            "python:psycopg" => TypeBuilder {
                type_overrides,
                enums,
                catalog: request.catalog.clone(),
                type_map: PYTHON_PSYCOPG.clone(),
            },
            _ => return Err(Error::NotSupportedLanguage(lang)),
        };
        Ok(resolver)
    }

    pub fn declared(&self, name: &str) -> Type {
        let name: Rc<str> = render(&self.type_map.new_type_case, context!(name=>name)).into();
        Type {
            declaration: name.clone(),
            annotation: name.clone(),
            constructor: name.clone(),
            import: Default::default(),
            pgtype_name: None,
            pgtype_schema: None,
        }
    }

    pub fn composite(&self, type_schema: &Rc<str>, type_name: &Rc<str>) -> Type {
        let ctx = &context! {type_schema => type_schema, type_name => type_name};
        let composite = &self.type_map.composite;
        Type {
            declaration: render(&composite.declaration, ctx).into(),
            annotation: render(&composite.annotation, ctx).into(),
            constructor: render(&composite.constructor, ctx).into(),
            import: composite
                .import
                .iter()
                .map(|import| render(import, ctx).into())
                .collect(),
            pgtype_name: Some(type_name.clone()),
            pgtype_schema: Some(type_schema.clone()),
        }
    }

    pub fn from_col(&self, column: &Column) -> Type {
        let mut type_ = self.from_column_type(&column.type_field);

        type_ = self.array(type_, column.type_field.array_dimensions);

        if let Some(enum_type) = self.try_enum(&column) {
            type_ = enum_type;
        }

        if column.is_nullable {
            return self.null(&type_);
        }

        return type_;
    }

    pub fn null(&self, type_: &Type) -> Type {
        let map = &self.type_map;
        let cx = context!(type=> type_);
        Type {
            annotation: render(&map.null.annotation, &cx).into(),
            declaration: render(&map.null.declaration, &cx).into(),
            constructor: render(&map.null.constructor, &cx).into(),
            import: type_.import.clone(),
            pgtype_name: type_.pgtype_name.clone(),
            pgtype_schema: type_.pgtype_schema.clone(),
        }
    }

    pub fn try_enum(&self, column: &Column) -> Option<Type> {
        let type_name = column.foreign_table_name.clone()?;
        let schema_name = column.foreign_table_schema.clone()?;
        let full_name = (schema_name.clone(), type_name.clone());

        if self.enums.contains(&full_name) {
            return Some(self.composite(&schema_name, &type_name));
        }
        return None;
    }

    pub fn array(&self, mut type_: Type, dim: i64) -> Type {
        let map = &self.type_map;
        if dim == 0 {
            return type_;
        }

        for _ in 0..dim {
            let cx = context!(type => type_);
            type_ = Type {
                annotation: render(&map.array.annotation, &cx).into(),
                declaration: render(&map.array.declaration, &cx).into(),
                constructor: render(&map.array.constructor, &cx).into(),
                import: type_.import.clone(),
                pgtype_name: type_.pgtype_name.clone(),
                pgtype_schema: type_.pgtype_schema.clone(),
            };
        }

        return Type {
            pgtype_name: type_.pgtype_name.map(|ty| format!("_{}", ty).into()),
            pgtype_schema: type_.pgtype_schema.map(|ty| format!("_{}", ty).into()),
            ..type_
        };
    }

    pub fn from_column_type(&self, ty: &ColumnType) -> Type {
        self.resolve(&ty.schema_name, &ty.name)
    }

    pub fn from_output_type(&self, ty: &OutputType) -> Type {
        return self.resolve(&ty.schema, &ty.name);
    }

    pub fn resolve(&self, type_schema: &Rc<str>, type_name: &Rc<str>) -> Type {
        if let Some(ty) = self.resolve_from_catalog(type_schema, type_name) {
            return ty;
        };

        if !type_name.starts_with('_') {
            return self.resolve_non_array(type_schema, type_name);
        }

        let type_name: Rc<str> = type_name.strip_prefix('_').unwrap().into();
        return self.array(self.resolve_non_array(&type_schema, &type_name), 1);
    }

    pub fn resolve_non_array(&self, type_schema: &Rc<str>, type_name: &Rc<str>) -> Type {
        if let Some(ty) = self.resolve_from_catalog(type_schema, type_name) {
            return ty;
        };

        let ty = self.resolve_type_config(&type_schema, &type_name);

        Type {
            declaration: ty.name.clone(),
            constructor: ty.name.clone(),
            annotation: ty.annotation.clone().unwrap_or(ty.name.clone()),
            import: ty.import.clone(),
            pgtype_name: Some(type_name.clone()),
            pgtype_schema: Some(type_schema.clone()),
        }
    }

    fn resolve_type_config(&self, type_schema: &str, type_name: &str) -> TypeConfig {
        let name = format!("{}.{}", type_schema, type_name);

        if let Some(value) = self.type_overrides.get(&*name) {
            return value.clone();
        }

        self.default_type_resolution(type_schema, type_name)
    }

    fn resolve_from_catalog(&self, type_schema: &Rc<str>, type_name: &Rc<str>) -> Option<Type> {
        let schema = self
            .catalog
            .schemas
            .iter()
            .find(|schema| &schema.name == type_schema)?;

        let model = schema
            .models
            .iter()
            .find(|model| &model.name == type_name)?;

        Some(Self::composite(&self, type_schema, type_name))
    }

    fn default_type_resolution(&self, type_schema: &str, type_name: &str) -> TypeConfig {
        self.type_map
            .schema
            .get(&*type_schema)
            .map(|schema| schema.get(&*type_name))
            .flatten()
            .cloned()
            .unwrap_or_else(|| self.type_map.wildcard.clone())
    }
}

const PYTHON_ASYNCPG: LazyLock<TypeMap> = LazyLock::new(|| {
    let json = include_str!("../templates/python:asyncpg/types.json");
    return serde_json::from_str(json).expect("failed to deserialize python:asyncpg/types.json ");
});

const PYTHON_PSYCOPG: LazyLock<TypeMap> = LazyLock::new(|| {
    let json = include_str!("../templates/python:psycopg/types.json");
    return serde_json::from_str(json).expect("failed to deserialize python:psycopg/types.json ");
});
