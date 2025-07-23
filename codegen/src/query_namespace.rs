use std::{collections::BTreeMap, rc::Rc};

use crate::{method::Method, utils::to_pascal_case};

pub struct QueryNamespace {
    pub name: String,
    pub subnamespaces: BTreeMap<Rc<str>, QueryNamespace>,
    pub methods: Vec<Method>,
}

impl QueryNamespace {
    pub fn new() -> QueryNamespace {
        QueryNamespace {
            name: "Queries".into(),
            subnamespaces: Default::default(),
            methods: Default::default(),
        }
    }

    pub fn resolve(&mut self, name: &str) -> &mut QueryNamespace {
        self._resolve(&name.split('.').collect::<Vec<_>>())
    }

    pub fn _resolve(&mut self, name: &[&str]) -> &mut QueryNamespace {
        if name.is_empty() {
            return self;
        }

        let entry = self.subnamespaces.entry(name[0].into());

        let namespace = entry.or_insert_with(|| QueryNamespace {
            name: to_pascal_case(name[0].into()) + "Queries",
            methods: Default::default(),
            subnamespaces: Default::default(),
        });

        return namespace._resolve(&name[1..]);
    }
}
