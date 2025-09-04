use std::sync::Arc;

use minijinja::value::{Enumerator, Object, ObjectRepr};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Debug, Serialize, Deserialize)]
#[serde(tag = "variant", content = "c")]
pub enum Type {
    // A type not matching any of these
    Other {
        schema: Arc<str>,
        name: Arc<str>,
    },

    // Uncategorized
    Bool,
    Uuid,

    // Text
    Text,
    VarChar,
    BpChar,
    Bytea,

    // Numeric types
    Int2,
    Int4,
    Int8,
    Serial2,
    Serial4,
    Serial8,
    Decimal,
    Numeric,
    Money,
    Float4,
    Float8,

    // Time types
    Timestamp,
    Date,
    Time,
    TimestampTz,
    DateTz,
    TimeTz,
    Range,
    Interval,

    // Range types
    Int4Range,
    Int8Range,
    NumRange,
    TsRange,
    TsTzRange,
    DateRange,
    DateMultiRange,
    Int4MultiRange,
    Int8MultiRange,
    NumMultiRange,
    TsMultiRange,
    TsTzMultiRange,

    // Geometric types
    Point,
    Line,
    LSeg,
    Box,
    Path,
    Polygon,
    Circle,

    // Generic types
    Nullable(Arc<Type>),
    Array {
        r#type: Arc<Type>,
        dim: i64,
    },

    // User defined types
    UserDefined {
        module_path: Arc<[Arc<str>]>,
        name: Arc<str>,
    },

    // Networking types
    Cid,
    Cidr,
    Inet,
    MacAddr,
    MacAddr8,

    // Bit string types
    Bit,
    BitVarying,

    // Text Seach types
    TsVector,
    TsQuery,

    // Encoding types
    Xml,
    Json,
    Jsonb,
    JsonPath,

    // PseudoTypes
    Any,
    AnyArray,
    AnyElement,
    AnyNonArray,
    AnyEnum,
    AnyRange,
    AnyMultiRange,
    AnyCompatible,
    AnyCompatibleArray,
    AnyCompatibleMultiRange,
    AnyCompatibleNonArray,
    AnycompatibleRange,
    Cstring,
    Record,
    Void,
    Unknown,
}

impl Type {
    #[rustfmt::skip]
    pub const NAMES: &[(&'static str, &'static str, Type)] = &[
        ("any", "pg_catalog.any", Type::Any),
        ("anyarray", "pg_catalog.anyarray", Type::AnyArray),
        ("anycompatible", "pg_catalog.anycompatible", Type::AnyCompatible),
        ("anycompatiblearray", "pg_catalog.anycompatiblearray", Type::AnyCompatibleArray),
        ("anycompatiblemultirange", "pg_catalog.anycompatiblemultirange", Type::AnyCompatibleMultiRange),
        ("anycompatiblenonarray", "pg_catalog.anycompatiblenonarray", Type::AnyCompatibleNonArray),
        ("anycompatiblerange", "pg_catalog.anycompatiblerange", Type::AnycompatibleRange),
        ("anyelement", "pg_catalog.anyelement", Type::AnyElement),
        ("anyenum", "pg_catalog.anyenum", Type::AnyEnum),
        ("anymultirange", "pg_catalog.anymultirange", Type::AnyMultiRange),
        ("anynonarray", "pg_catalog.anynonarray", Type::AnyNonArray),
        ("anyrange", "pg_catalog.anyrange", Type::AnyRange),
        ("bit", "pg_catalog.bit", Type::Bit),
        ("bitvarying", "pg_catalog.bitvarying", Type::BitVarying),
        ("bool", "pg_catalog.bool", Type::Bool),
        ("box", "pg_catalog.box", Type::Box),
        ("bpchar", "pg_catalog.bpchar", Type::BpChar),
        ("bytea", "pg_catalog.bytea", Type::Bytea),
        ("cid", "pg_catalog.cid", Type::Cid),
        ("cidr", "pg_catalog.cidr", Type::Cidr),
        ("circle", "pg_catalog.circle", Type::Circle),
        ("cstring", "pg_catalog.cstring", Type::Cstring),
        ("date", "pg_catalog.date", Type::Date),
        ("datemultirange", "pg_catalog.datemultirange", Type::DateMultiRange),
        ("daterange", "pg_catalog.daterange", Type::DateRange),
        ("datetz", "pg_catalog.datetz", Type::DateTz),
        ("decimal", "pg_catalog.decimal", Type::Decimal),
        ("float4", "pg_catalog.float4", Type::Float4),
        ("float8", "pg_catalog.float8", Type::Float8),
        ("inet", "pg_catalog.inet", Type::Inet),
        ("int2", "pg_catalog.int2", Type::Int2),
        ("int4", "pg_catalog.int4", Type::Int4),
        ("int4multirange", "pg_catalog.int4multirange", Type::Int4MultiRange),
        ("int4range", "pg_catalog.int4range", Type::Int4Range),
        ("int8", "pg_catalog.int8", Type::Int8),
        ("int8multirange", "pg_catalog.int8multirange", Type::Int8MultiRange),
        ("int8range", "pg_catalog.int8range", Type::Int8Range),
        ("interval", "pg_catalog.interval", Type::Interval),
        ("json", "pg_catalog.json", Type::Json),
        ("jsonb", "pg_catalog.jsonb", Type::Jsonb),
        ("jsonpath", "pg_catalog.jsonpath", Type::JsonPath),
        ("line", "pg_catalog.line", Type::Line),
        ("lseg", "pg_catalog.lseg", Type::LSeg),
        ("macaddr", "pg_catalog.macaddr", Type::MacAddr),
        ("macaddr8", "pg_catalog.macaddr8", Type::MacAddr8),
        ("money", "pg_catalog.money", Type::Money),
        ("numeric", "pg_catalog.numeric", Type::Numeric),
        ("nummultirange", "pg_catalog.nummultirange", Type::NumMultiRange),
        ("numrange", "pg_catalog.numrange", Type::NumRange),
        ("path", "pg_catalog.path", Type::Path),
        ("point", "pg_catalog.point", Type::Point),
        ("polygon", "pg_catalog.polygon", Type::Polygon),
        ("range", "pg_catalog.range", Type::Range),
        ("record", "pg_catalog.record", Type::Record),
        ("serial2", "pg_catalog.serial2", Type::Serial2),
        ("serial4", "pg_catalog.serial4", Type::Serial4),
        ("serial8", "pg_catalog.serial8", Type::Serial8),
        ("text", "pg_catalog.text", Type::Text),
        ("time", "pg_catalog.time", Type::Time),
        ("timestamp", "pg_catalog.timestamp", Type::Timestamp),
        ("timestamptz", "pg_catalog.timestamptz", Type::TimestampTz),
        ("timetz", "pg_catalog.timetz", Type::TimeTz),
        ("tsmultirange", "pg_catalog.tsmultirange", Type::TsMultiRange),
        ("tsquery", "pg_catalog.tsquery", Type::TsQuery),
        ("tsrange", "pg_catalog.tsrange", Type::TsRange),
        ("tstzmultirange", "pg_catalog.tstzmultirange", Type::TsTzMultiRange),
        ("tstzrange", "pg_catalog.tstzrange", Type::TsTzRange),
        ("tsvector", "pg_catalog.tsvector", Type::TsVector),
        ("unknown", "pg_catalog.unknown", Type::Unknown),
        ("uuid", "pg_catalog.uuid", Type::Uuid),
        ("varchar", "pg_catalog.varchar", Type::VarChar),
        ("void", "pg_catalog.void", Type::Void),
        ("xml", "pg_catalog.xml", Type::Xml),
    ];
}

impl Type {
    pub fn from_jinja(value: minijinja::Value) -> Self {
        let deserializer = serde::de::value::MapDeserializer::new(
            value.as_object().unwrap().try_iter_pairs().unwrap(),
        );
        Type::deserialize(deserializer).unwrap()
    }
}

#[test]
fn array_is_sorted() {
    assert!(Type::NAMES.is_sorted())
}
#[test]
fn type_from_jinja() {
    let value = minijinja::Value::from_serialize(Type::Polygon);
    assert_eq!(Type::from_jinja(value), Type::Polygon)
}
