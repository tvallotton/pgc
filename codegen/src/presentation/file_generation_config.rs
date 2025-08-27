use minijinja::Environment;

use crate::{error::Error, presentation::type_mapping_service::TypeMapService, response::File};

#[derive(Clone)]
pub struct TemplateGenConfig {
    pub query_directory_entrypoint: &'static str,
    pub model_directory_entrypoint: &'static str,
    pub file_extension: &'static str,
    pub query_template: &'static str,
    pub model_template: &'static str,
    pub model_init_template: &'static str,
    pub type_map_service: &'static dyn TypeMapService,
    pub other_templates: Vec<File>,
    pub register_filters: Option<fn(&mut Environment) -> Result<(), Error>>,
}
