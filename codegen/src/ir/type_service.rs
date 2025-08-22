use std::rc::Rc;

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
                r#type: Rc::new(r#type),
                dim: column.type_field.array_dimensions,
            };
        }

        if column.is_nullable {
            r#type = Type::Nullable(Rc::new(r#type));
        }

        return r#type;
    }

    fn find_table_backed_enum(&self, column: &Column) -> Option<Type> {
        let schema = self.get_schema(column.foreign_table_schema.as_deref()?)?;
        self.resolve_enum(schema, column.foreign_table_name.as_ref()?)
    }

    fn resolve_from_catalog(&self, schema_name: &Rc<str>, name: &Rc<str>) -> Type {
        if &**schema_name == "pg_catalog" {
            return self.from_pg_catalog(&name);
        }
        self.from_user_defined_catalog(schema_name, name)
            .unwrap_or(Type::Any)
    }

    fn from_user_defined_catalog(&self, schema_name: &Rc<str>, name: &Rc<str>) -> Option<Type> {
        let schema = self.get_schema(schema_name)?;

        self.resolve_record(schema, name)
            .or_else(|| self.resolve_enum(schema, name))
    }

    fn resolve_enum(&self, schema: &Schema, name: &Rc<str>) -> Option<Type> {
        schema.enums.iter().find(|enum_| enum_.name == *name)?;
        Some(self.user_defined_model(schema, name))
    }

    fn resolve_record(&self, schema: &Schema, name: &Rc<str>) -> Option<Type> {
        schema.records.iter().find(|record| record.name == *name)?;
        Some(self.user_defined_model(schema, name))
    }

    fn user_defined_model(&self, schema: &Schema, name: &Rc<str>) -> Type {
        let module_path = Rc::new(["models".into(), schema.name.clone()]);
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

    fn from_pg_catalog(&self, name: &str) -> Type {
        match name {
            "bool" => Type::Bool,
            "uuid" => Type::Uuid,
            "text" => Type::Text,
            "varchar" => Type::VarChar,
            "bpchar" => Type::BpChar,
            "bytea" => Type::Bytea,
            "int2" => Type::Int2,
            "int4" => Type::Int4,
            "int8" => Type::Int8,
            "serial2" => Type::Serial2,
            "serial4" => Type::Serial4,
            "serial8" => Type::Serial8,
            "decimal" => Type::Decimal,
            "numeric" => Type::Numeric,
            "money" => Type::Money,
            "float4" => Type::Float4,
            "float8" => Type::Float8,
            "timestamp" => Type::Timestamp,
            "date" => Type::Date,
            "time" => Type::Time,
            "timestamptz" => Type::TimestampTz,
            "datetz" => Type::DateTz,
            "timetz" => Type::TimeTz,
            "range" => Type::Range,
            "interval" => Type::Interval,
            "int4range" => Type::Int4Range,
            "int8range" => Type::Int8Range,
            "numrange" => Type::NumRange,
            "tsrange" => Type::TsRange,
            "tstzrange" => Type::TsTzRange,
            "daterange" => Type::DateRange,
            "datemultirange" => Type::DateMultiRange,
            "int4multirange" => Type::Int4MultiRange,
            "int8multirange" => Type::Int8MultiRange,
            "nummultirange" => Type::NumMultiRange,
            "tsmultirange" => Type::TsMultiRange,
            "tstzmultirange" => Type::TsTzMultiRange,
            "point" => Type::Point,
            "line" => Type::Line,
            "lseg" => Type::LSeg,
            "box" => Type::Box,
            "path" => Type::Path,
            "polygon" => Type::Polygon,
            "circle" => Type::Circle,
            "cid" => Type::Cid,
            "cidr" => Type::Cidr,
            "inet" => Type::Inet,
            "macaddr" => Type::MacAddr,
            "macaddr8" => Type::MacAddr8,
            "bit" => Type::Bit,
            "bitvarying" => Type::BitVarying,
            "tsvector" => Type::TsVector,
            "tsquery" => Type::TsQuery,
            "xml" => Type::Xml,
            "json" => Type::Json,
            "jsonb" => Type::Jsonb,
            "jsonpath" => Type::JsonPath,
            "any" => Type::Any,
            "anyarray" => Type::AnyArray,
            "anyelement" => Type::AnyElement,
            "anynonarray" => Type::AnyNonArray,
            "anyenum" => Type::AnyEnum,
            "anyrange" => Type::AnyRange,
            "anymultirange" => Type::AnyMultiRange,
            "anycompatible" => Type::AnyCompatible,
            "anycompatiblearray" => Type::AnyCompatibleArray,
            "anycompatiblemultirange" => Type::AnyCompatibleMultiRange,
            "anycompatiblenonarray" => Type::AnyCompatibleNonArray,
            "anycompatiblerange" => Type::AnycompatibleRange,
            "cstring" => Type::Cstring,
            "internal" => Type::Internal,
            "record" => Type::Record,
            "void" => Type::Void,
            "unknown" => Type::Unknown,
            _ => Type::Any,
        }
    }
}
