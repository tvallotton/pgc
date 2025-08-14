use std::rc::Rc;

use minijinja::Environment;

use crate::{error::Error, request::Request, utils};

pub struct JinjaEnvironmentBuilder {
    pub target: Rc<str>,
}
pub const QUERY: &'static str = "query";
pub const MODEL_SCHEMA_FILE: &'static str = "model_schema_file";
pub const MODELS_DIR_ENTRYPOINT: &'static str = "model_dir_entrypoint";

impl JinjaEnvironmentBuilder {
    pub fn new(request: &Request) -> Self {
        Self {
            target: request.config.codegen.target.clone(),
        }
    }

    fn query_template(&self) -> Result<&'static str, Error> {
        Ok(match &*self.target {
            "python:asyncpg" => include_str!("../templates/python:asyncpg/query.py.jinja2"),
            "python:psycopg" => include_str!("../templates/python:psycopg/query.py.jinja2"),
            "typescript:postgres" => include_str!("../templates/typescript:postgres/query.jinja2"),
            _ => return Err(Error::NotSupportedLanguage(self.target.clone())),
        })
    }

    fn model_template(&self) -> Result<&'static str, Error> {
        Ok(match &*self.target {
            "python:asyncpg" => include_str!("../templates/python:asyncpg/model.py.jinja2"),
            "python:psycopg" => include_str!("../templates/python:psycopg/model.py.jinja2"),
            "typescript:postgres" => include_str!("../templates/typescript:postgres/model.jinja2"),
            _ => return Err(Error::NotSupportedLanguage(self.target.clone())),
        })
    }

    fn model_dir_entrypoint_template(&self) -> Result<&'static str, Error> {
        Ok(match &*self.target {
            "python:asyncpg" => include_str!("../templates/python:asyncpg/model_init.py.jinja2"),
            "python:psycopg" => include_str!("../templates/python:psycopg/model_init.py.jinja2"),
            "typescript:postgres" => {
                include_str!("../templates/typescript:postgres/model_init.jinja2")
            }
            _ => return Err(Error::NotSupportedLanguage(self.target.clone())),
        })
    }

    pub fn build(self) -> Result<Environment<'static>, Error> {
        let mut environment = utils::env();
        environment.add_template(QUERY, self.query_template()?)?;
        environment.add_template(MODEL_SCHEMA_FILE, self.model_template()?)?;
        environment.add_template(MODELS_DIR_ENTRYPOINT, self.model_dir_entrypoint_template()?)?;
        Ok(environment)
    }
}
