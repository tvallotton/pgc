use serde::{Deserialize, Serialize};

use crate::{error::Error, request::Request};

#[derive(Deserialize, Serialize, Clone)]
pub struct FileGenConfig {
    pub extension: String,
    pub directory_entrypoint: Option<String>,
    pub model_dir_entrypoint: Option<String>,
}

impl FileGenConfig {
    pub fn new(request: &Request) -> Result<FileGenConfig, Error> {
        let target = &request.config.codegen.target;
        let json = match &**target {
            "python:asyncpg" => include_str!("../templates/python:asyncpg/config.json"),
            "python:psycopg" => include_str!("../templates/python:psycopg/config.json"),
            "typescript:postgres" => include_str!("../templates/typescript:postgres/config.json"),
            _ => return Err(Error::NotSupportedLanguage(target.clone())),
        };
        Ok(serde_json::from_str(json).unwrap())
    }
}
