use std::{collections::BTreeMap, mem::take, rc::Rc};

use indexmap::IndexMap;

use crate::{
    method::{Method, MethodModel},
    r#type::Type,
    request::Query,
    type_builder::TypeBuilder,
};

pub struct MethodBuilder {
    type_builder: TypeBuilder,
    arguments: IndexMap<Rc<str>, Type>,
    input_models: BTreeMap<Rc<str>, MethodModel>,
}

impl MethodBuilder {
    pub fn new(type_builder: TypeBuilder) -> Self {
        MethodBuilder {
            type_builder,
            arguments: Default::default(),
            input_models: Default::default(),
        }
    }

    pub fn build(&mut self, query: &Query) -> Method {
        self.init_input_models(query);
        Method {
            query: query.clone(),
            arguments: take(&mut self.arguments),
            input_models: take(&mut self.input_models),
            output_type: self.output_type(query),
            output_model: self.output_model(query),
        }
    }

    pub fn init_input_models(&mut self, query: &Query) {
        for param in query.parameters.iter() {
            let mut ty = self.type_builder.from_query_type(&param.type_);

            if !param.not_null {
                ty = self.type_builder.null(&ty);
            }

            if let Some((record, field)) = param.name.split_once('.') {
                self.include_input_model(record, field, ty, query);

                continue;
            };

            self.arguments.insert(param.name.clone(), ty);
        }
    }

    pub fn include_input_model(&mut self, record: &str, field: &str, ty: Type, query: &Query) {
        let query_name = query.name.clone();
        let type_builder = self.type_builder.clone();
        let entry = self.input_models.entry(record.into());

        let query_model = entry.or_insert_with(|| MethodModel {
            r#type: type_builder.declared(&format!("{}_{}", query_name, record)),
            fields: IndexMap::default(),
        });

        query_model.fields.insert(field.into(), ty);

        self.arguments
            .insert(record.into(), query_model.r#type.clone());
    }

    fn output_type(&self, query: &Query) -> Option<Type> {
        if &*query.command == "exec" {
            return None;
        }

        if query.output.len() == 0 {
            return None;
        }

        if query.output.len() == 1 {
            let pg_type = &query.output[0].type_;
            let output_type = self.type_builder.from_query_type(&pg_type);
            return Some(output_type);
        }

        Some(self.type_builder.declared(&format!("{}_row", query.name)))
    }

    fn output_model(&self, query: &Query) -> Option<MethodModel> {
        if query.output.len() < 2 {
            return None;
        }
        let columns = query
            .output
            .iter()
            .map(|column| {
                let type_ = self.type_builder.from_query_type(&column.type_);
                (column.name.clone(), type_)
            })
            .collect();

        Some(MethodModel {
            r#type: self.output_type(query)?,
            fields: columns,
        })
    }
}
