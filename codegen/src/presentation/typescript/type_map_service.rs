use std::sync::Arc;

use crate::{
    ir::Type,
    presentation::type_mapping_service::{LanguageType, TypeMapService},
    utils::{to_camel_case, to_pascal_case},
};

#[derive(Clone, Copy)]
pub struct TypescriptTypeMapService;

impl TypescriptTypeMapService {
    pub fn column_requires_parser(&self, r#type: Type) -> bool {
        return matches!(
            r#type,
            Type::Int8 | Type::Serial8 | Type::UserDefined { .. } | Type::Array { .. }
        );
    }

    pub fn type_parser(&self, r#type: Type) -> String {
        match r#type {
            Type::Nullable(r#type) => format!(
                "new NullParser({})",
                self.type_parser(Type::clone(&*r#type))
            ),
            Type::UserDefined { module_path, name } => {
                format!("parser.{}.{}()", module_path[1], to_camel_case(&name))
            }

            Type::Array { r#type, dim } if dim != 1 => {
                format!(
                    "{}.arrayOfThis()",
                    self.type_parser(Type::Array {
                        r#type,
                        dim: dim - 1
                    })
                )
            }
            Type::Array { r#type, dim: 1 } => {
                format!(
                    "new ArrayParser({})",
                    self.type_parser(Type::clone(&*r#type))
                )
            }
            Type::Bool => "new BooleanParser()".into(),
            Type::Date | Type::DateTz | Type::Timestamp | Type::TimestampTz => {
                "new DateParser()".into()
            }
            Type::Int8 => "new BigIntParser()".into(),
            Type::Float4 | Type::Float8 | Type::Int2 | Type::Int4 => "new NumberParser()".into(),
            Type::Bytea => "new BufferParser()".into(),
            Type::Json => "new JsonParser()".into(),
            _ => "new StringParser()".into(),
        }
    }
}

impl TypeMapService for TypescriptTypeMapService {
    fn get(&self, current_module: Vec<String>, r#type: &Type) -> LanguageType {
        match r#type {
            Type::Any | Type::AnyCompatibleNonArray | Type::AnyCompatible => {
                LanguageType::annotation("any")
            }
            Type::Int8 => LanguageType::annotation("bigint"),
            Type::AnyArray | Type::AnyCompatibleArray => LanguageType::annotation("Array<any>"),
            Type::Json => LanguageType::annotation("any"),
            Type::UserDefined { module_path, name } => {
                let name: Arc<str> = to_pascal_case(&name).into();
                let module: Arc<_> = module_path.join(".").into();
                let mut annotation = format!("{module}.{name}").into();
                let same_module = current_module
                    .iter()
                    .map(|s| &**s)
                    .eq(module_path.iter().map(|s| &**s));
                if same_module {
                    annotation = name.clone();
                }

                LanguageType {
                    name: Some(name.clone()),
                    annotation,
                    import: vec![],
                    module: Some(module),
                }
            }
            Type::Nullable(r#type) => {
                let r#type = self.get(current_module, r#type);
                LanguageType {
                    name: r#type.name,
                    annotation: format!("{} | null", r#type.annotation).into(),
                    import: r#type.import,
                    module: r#type.module,
                }
            }
            Type::Array { r#type, dim } => {
                let r#type = self.get(current_module, r#type);
                let mut annotation = r#type.annotation;
                for _ in 0..*dim {
                    annotation = format!("Array<{}>", annotation).into();
                }
                LanguageType {
                    name: None,
                    annotation,
                    import: r#type.import,
                    module: r#type.module,
                }
            }
            Type::Bool => LanguageType::annotation("boolean").name("boolean"),
            Type::Bytea => LanguageType::annotation("Buffer").name("Buffer"),
            Type::Int2 | Type::Int4 | Type::Float4 | Type::Float8 => {
                LanguageType::annotation("number").name("Date")
            }
            Type::Date | Type::DateTz | Type::Timestamp | Type::TimestampTz => {
                LanguageType::annotation("Date").name("Date")
            }
            _ => LanguageType::annotation("string").name("string"),
        }
    }
}
