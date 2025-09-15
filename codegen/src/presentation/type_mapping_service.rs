use std::{collections::BTreeMap, sync::Arc};

use serde::Deserialize;

use crate::{
    ir::{Ir, Type},
    request::TypeConfig,
};

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LanguageType {
    pub name: Option<Arc<str>>,
    pub annotation: Arc<str>,
    pub import: Vec<Arc<str>>,
    pub constructor: Option<Arc<str>>,
}

pub trait TypeMapService: Send + Sync + 'static {
    fn get(&self, module: Vec<String>, r#type: &Type) -> LanguageType;
}

pub struct OverriddenTypeMapService {
    service: &'static dyn TypeMapService,
    overrides: Arc<BTreeMap<Arc<str>, TypeConfig>>,
}

impl OverriddenTypeMapService {
    pub fn new(ir: Ir, service: &'static dyn TypeMapService) -> Self {
        Self {
            service,
            overrides: ir.request.config.codegen.types.clone(),
        }
    }

    fn get_from_ident(&self, module: Vec<String>, r#type: &Type, ident: &str) -> LanguageType {
        let Some(type_config) = self.overrides.get(ident) else {
            return self.service.get(module, r#type);
        };

        return LanguageType {
            name: None,
            annotation: type_config.annotation.clone(),
            constructor: None,
            import: type_config.import.iter().cloned().collect(),
        };
    }
}

impl TypeMapService for OverriddenTypeMapService {
    fn get(&self, module: Vec<String>, r#type: &Type) -> LanguageType {
        if let Type::Other { schema, name } = r#type {
            return self.get_from_ident(module, r#type, &format!("{}.{}", schema, name));
        }

        let Ok(index) = Type::NAMES.binary_search_by(|(_, _, ty)| ty.cmp(r#type)) else {
            return self.service.get(module, r#type);
        };

        let (_, name, _) = Type::NAMES[index];

        self.get_from_ident(module, r#type, name)
    }
}

impl LanguageType {
    pub fn annotation(annotation: &str) -> Self {
        LanguageType {
            annotation: annotation.into(),
            name: None,
            import: vec![],
            constructor: None,
        }
    }

    pub fn name(self, name: &str) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }

    pub fn import<const N: usize>(self, import: [&str; N]) -> Self {
        let import: Vec<Arc<str>> = import.into_iter().map(Into::into).collect();
        Self { import, ..self }
    }
}
