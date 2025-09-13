use std::sync::Arc;

use crate::{
    ir::Type,
    presentation::type_mapping_service::{LanguageType, TypeMapService},
    utils::{to_camel_case, to_pascal_case},
};

#[derive(Clone, Copy)]
pub struct TypescriptTypeMapService;

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
                let module: Arc<str> = module_path.join(".").into();
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
                    constructor: Some(annotation.clone()),
                    annotation,
                    import: vec![],
                }
            }
            Type::Nullable(r#type) => {
                let r#type = self.get(current_module, r#type);
                LanguageType {
                    name: r#type.name,
                    annotation: format!("{} | null", r#type.annotation).into(),
                    import: r#type.import,
                    constructor: r#type.constructor.clone(),
                }
            }
            Type::Array { r#type, dim } => {
                let r#type = self.get(current_module, r#type);
                let mut annotation = r#type.annotation;
                let mut constructor = r#type.constructor.clone();
                for i in 0..*dim {
                    annotation = format!("Array<{}>", annotation).into();

                    let Some(prev_constructor) = constructor else {
                        continue;
                    };

                    constructor =
                        Some(format!("((arr{i}) => arr{i}.map({prev_constructor}))").into())
                }
                LanguageType {
                    name: None,
                    annotation,
                    import: r#type.import,
                    constructor,
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
