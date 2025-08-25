use crate::{ir::Ir, request::Request};

mod python;
mod typescript;

mod file_generation_config;
mod file_generator;
mod type_mapping_service;

mod environment;

// pub fn generate_files(ir: Ir) {
//     match ir.request.config.codegen {
//         "python" =>
//     }
// }
