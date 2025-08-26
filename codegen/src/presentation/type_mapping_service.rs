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
    pub module: Option<Arc<str>>,
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
}

impl TypeMapService for OverriddenTypeMapService {
    fn get(&self, module: Vec<String>, r#type: &Type) -> LanguageType {
        return self.service.get(module, r#type);
        // let Ok(ty) = Type::NAMES.binary_search_by(|(_, _, ty)| ty.cmp(r#type)) else {
        // };
        // let (_, name, _) = Type::NAMES[ty];

        // let Some(type_config) = self.overrides.get(name) else {
        //     return self.service.get(module, r#type);
        // };

        // return LanguageType {
        //     name: None,
        //     annotation: type_config.annotation.clone(),
        //     import: type_config.import.clone(),
        //     module: None,
        // };
    }
}

impl LanguageType {
    pub fn annotation(annotation: &str) -> Self {
        LanguageType {
            annotation: annotation.into(),
            name: None,
            import: vec![],
            module: None,
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

    pub fn module(self, module: &str) -> Self {
        Self {
            module: Some(module.into()),
            ..self
        }
    }
}
