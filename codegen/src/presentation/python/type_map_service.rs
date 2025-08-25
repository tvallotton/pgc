use crate::{
    ir::Type,
    presentation::type_mapping_service::{LanguageType, TypeMapService},
};
use std::sync::Arc;

struct AsyncpgTypeMapService;
pub struct PsycopgTypeMapService;

impl TypeMapService for PsycopgTypeMapService {
    #[rustfmt::skip]
    fn get(&self, module: Arc<[Arc<str>]>, r#type: &Type) -> LanguageType {
        match r#type {
            Type::Bit
            | Type::BitVarying
            | Type::Record
            | Type::Line
            | Type::LSeg
            | Type::Point
            | Type::Path
            | Type::Polygon
            | Type::Circle
            | Type::Box => LanguageType::annotation("str"),
            Type::AnyMultiRange | Type::AnyCompatibleMultiRange => LanguageType::annotation("list[psycopg.types.range.Range]").import(["import psycopg.types.range"]),
            Type::TsMultiRange | Type::TsTzMultiRange => LanguageType::annotation("list[psycopg.types.range.Range[datetime.datetime]]").import(["import psycopg.types.range", "import datetime"]),
            Type::DateMultiRange  => LanguageType::annotation("list[psycopg.types.range.Range[datetime.date]]").import(["import psycopg.types.range", "import datetime"]),
            Type::DateRange => LanguageType::annotation("psycopg.types.range.Range[datetime.date]").import(["import psycopg.types.range", "import datetime"]),
            Type::TsRange | Type::TsTzRange => LanguageType::annotation("psycopg.types.range.Range[datetime.datetime]").import(["import psycopg.types.range", "import datetime"]),
            Type::NumMultiRange => LanguageType::annotation("list[psycopg.types.range.Range[decimal.Decimal]]").import(["import psycopg.types.range", "import decimal"]),
            Type::Int4Range | Type::Int8Range => LanguageType::annotation("psycopg.types.range.Range[int]").import(["import psycopg.types.range"]),
            Type::Range | Type::AnyRange | Type::AnycompatibleRange => LanguageType::annotation("psycopg.types.range.Range").import(["import psycopg.types.range"]),
            Type::NumRange => LanguageType::annotation("psycopg.types.range.Range[decimal.Decimal]").import(["import psycopg.types.range", "import decimal"]),
            Type::Int4MultiRange | Type::Int8MultiRange  => LanguageType::annotation("list[psycopg.types.range.Range[int]]").import(["import psycopg.types.range"]),
            _ => return AsyncpgTypeMapService.get(module, r#type),
        }
    }
}

impl TypeMapService for AsyncpgTypeMapService {
    #[rustfmt::skip]
    fn get(&self, current_module: Arc<[Arc<str>]>, r#type: &crate::ir::Type) -> LanguageType {
        match r#type {
            Type::UserDefined { module_path, name }  => {
                let module: Arc<_> = module_path.join(".").into();
                LanguageType { name: Some(name.clone()), annotation: format!("{module}.{name}").into(), import: vec![format!("import {}", module).into()], module: Some(module) }
            },
            Type::Nullable(r#type) => {
                let r#type = self.get(current_module, r#type);
                LanguageType {
                    name: r#type.name,
                    annotation: format!("{} | None", r#type.annotation).into(),
                    import: r#type.import,
                    module: r#type.module
                }
            }
            Type::Array { r#type, dim } => {
                let r#type = self.get(current_module, r#type);
                let mut annotation = r#type.annotation;
                for _ in 0..*dim {
                    annotation = format!("list[{}]", annotation).into();
                }
                LanguageType {
                    name: None,
                    annotation,
                    import: r#type.import,
                    module: r#type.module
                }
            }
            Type::AnyArray | Type::AnyCompatibleArray => LanguageType::annotation("list"),
            Type::Void => LanguageType::annotation("None"),
            Type::Bool => LanguageType::annotation("bool"),
            Type::Bytea => LanguageType::annotation("bytes"),
            Type::Cidr => LanguageType::annotation("ipaddress.IPv4Network | ipaddress.IPv6Network").import(["import ipaddress"]),
            Type::Inet => LanguageType::annotation("ipaddress.IPv4Interface | ipaddress.IPv6Interface").import(["import ipaddress"]),
            Type::Date | Type::DateTz => LanguageType::annotation("datetime.date").import(["import datetime"]),
            Type::Time | Type::TimeTz=> LanguageType::annotation("datetime.time").import(["import datetime"]),
            Type::Timestamp | Type::TimestampTz => LanguageType::annotation("datetime.datetime").import(["import datetime"]),
            Type::Interval => LanguageType::annotation("datetime.timedelta").import(["import datetime"]),
            Type::Float4 | Type::Float8 => LanguageType::annotation("float"),
            Type::Uuid => LanguageType::annotation("uuid.UUID").import(["import uuid"]),
            Type::Record => LanguageType::annotation("asyncpg.Record").import(["import asyncpg"]),
            Type::Bit | Type::BitVarying => LanguageType::annotation("asyncpg.BitString").import(["import asyncpg"]),
            Type::Box => LanguageType::annotation("asyncpg.Box").import(["import asyncpg"]),
            Type::Int4Range | Type::Int8Range  => LanguageType::annotation("asyncpg.Range[int]").import(["import asyncpg"]),
            Type::NumRange => LanguageType::annotation("asyncpg.Range[decimal.Decimal]").import(["import asyncpg", "import decimal"]),
            Type::Int4MultiRange | Type::Int8MultiRange  => LanguageType::annotation("list[asyncpg.Range[int]]").import(["import asyncpg"]),
            Type::NumMultiRange => LanguageType::annotation("list[asyncpg.Range[decimal.Decimal]]").import(["import asyncpg", "import decimal"]),
            Type::Circle => LanguageType::annotation("asyncpg.Circle").import(["import asyncpg"]),
            Type::Line => LanguageType::annotation("asyncpg.Line").import(["import asyncpg"]),
            Type::LSeg => LanguageType::annotation("asyncpg.LineSegment").import(["import asyncpg"]),
            Type::Path => LanguageType::annotation("asyncpg.Path").import(["import asyncpg"]),
            Type::Point => LanguageType::annotation("asyncpg.Point").import(["import asyncpg"]),
            Type::Polygon => LanguageType::annotation("asyncpg.Polygon").import(["import asyncpg"]),
            Type::AnyRange
            | Type::TsRange
            | Type::TsTzRange
            | Type::DateRange
            | Type::AnycompatibleRange
            | Type::Range => LanguageType::annotation("asyncpg.Range").import(["import asyncpg"]),
            Type::AnyMultiRange
            | Type::AnyCompatibleMultiRange
            | Type::TsMultiRange
            | Type::TsTzMultiRange
            | Type::DateMultiRange => LanguageType::annotation("list[asyncpg.Range]").import(["import asyncpg"]),
            Type::Any
            | Type::Unknown
            | Type::AnyElement
            | Type::AnyNonArray
            | Type::AnyCompatibleNonArray
            | Type::AnyCompatible => LanguageType::annotation("typing.Any").import(["import typing"]),
            Type::BpChar
            | Type::VarChar
            | Type::Text
            | Type::Xml
            | Type::Json
            | Type::Jsonb
            | Type::Cstring
            | Type::Money
            | Type::AnyEnum
            | Type::JsonPath
            | Type::Cid
            | Type::MacAddr
            |  Type::MacAddr8
            | Type::TsVector
            | Type::TsQuery
            | Type::Other {..} => LanguageType::annotation("str"),
            Type::Int2
            | Type::Int4
            | Type::Int8
            | Type::Serial2
            | Type::Serial4
            | Type::Serial8 => LanguageType::annotation("int"),
            Type::Numeric | Type::Decimal => LanguageType::annotation("decimal.Decimal").import(["import decimal"]),
        }
    }
}
