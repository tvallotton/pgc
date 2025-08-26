use crate::{
    error::Error,
    ir::Ir,
    presentation::{templating_service::TemplatingService, type_mapping_service::TypeMapService},
    request::Codegen,
    response::File,
};

mod python;
mod typescript;

mod file_generation_config;
mod templating_service;
mod type_mapping_service;

mod environment;

pub struct PresentationService {
    pub ir: Ir,
}

trait FileGeneratorService {
    fn generate(&self) -> Result<Vec<File>, Error>;
}

impl PresentationService {
    pub fn generate(&self) -> Result<Vec<File>, Error> {
        self.templating_service()?.generate()
    }

    fn templating_service(&self) -> Result<TemplatingService, Error> {
        let Codegen {
            language, driver, ..
        } = self.ir.request.config.codegen.clone();

        let config = match (&*language, &*driver) {
            ("python", "asyncpg") => python::asyncpg(&self.ir)?,
            ("python", "psycopg") => python::psycopg(&self.ir)?,
            ("python", _) => return Err(Error::UnsupportedLanguage(language)),
            _ => return Err(Error::UnsupportedLanguage(language)),
        };

        TemplatingService::new(self.ir.clone(), config)
    }

    pub fn type_map_service(&self) -> Result<&'static dyn TypeMapService, Error> {
        let Codegen {
            language, driver, ..
        } = self.ir.request.config.codegen.clone();

        match (&*language, &*driver) {
            ("python", "asyncpg") => Ok(&python::AsyncpgTypeMapService),
            ("python", "psycopg") => Ok(&python::PsycopgTypeMapService),
            ("python", _) => return Err(Error::UnsupportedLanguage(language)),
            _ => return Err(Error::UnsupportedLanguage(language)),
        }
    }
}
