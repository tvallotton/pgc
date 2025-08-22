use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{
    ir::model_modules::Model,
    request::{Column, Enum, Record},
};

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Debug, Serialize, Deserialize)]
pub enum Type {
    // A type not matching any of these
    Other {
        schema: Rc<str>,
        name: Rc<str>,
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
    Nullable(Rc<Type>),
    Array {
        r#type: Rc<Type>,
        dim: i64,
    },

    // User defined types
    UserDefined {
        module_path: Rc<[Rc<str>]>,
        name: Rc<str>,
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
    Internal,
    Record,
    Void,
    Unknown,
}

impl Type {}
