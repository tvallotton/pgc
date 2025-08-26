pub use type_map_service::{AsyncpgTypeMapService, PsycopgTypeMapService};

use crate::{error::Error, ir::Ir, presentation::file_generation_config::TemplateGenConfig};

pub(super) mod driver;
pub mod type_map_service;

pub fn asyncpg(ir: &Ir) -> Result<TemplateGenConfig, Error> {
    check_required_options(&ir)?;
    Ok(TemplateGenConfig {
        query_directory_entrypoint: "__init__.py",
        model_directory_entrypoint: "__init__.py",
        file_extension: "py",
        query_template: include_str!("./templates/asyncpg/query.j2"),
        model_template: include_str!("./templates/asyncpg/model.j2"),
        model_init_template: include_str!("./templates/asyncpg/model_init.j2"),
        type_map_service: &AsyncpgTypeMapService,
        static_files: &[],
    })
}

pub fn psycopg(ir: &Ir) -> Result<TemplateGenConfig, Error> {
    check_required_options(&ir)?;
    Ok(TemplateGenConfig {
        query_directory_entrypoint: "__init__.py",
        model_directory_entrypoint: "__init__.py",
        file_extension: "py",
        query_template: include_str!("./templates/psycopg/query.j2"),
        model_template: include_str!("./templates/psycopg/model.j2"),
        model_init_template: include_str!("./templates/psycopg/model_init.j2"),
        type_map_service: &PsycopgTypeMapService,
        static_files: &[],
    })
}

pub fn check_required_options(ir: &Ir) -> Result<(), Error> {
    ir.request
        .config
        .codegen
        .options
        .get("package")
        .ok_or(Error::MissingConfigurationOption {
            language: "python",
            option: "package",
        })?;
    Ok(())
}
