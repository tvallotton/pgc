use minijinja::{Environment, Value};

use crate::{
    error::Error,
    ir::Type,
    presentation::{
        file_generation_config::TemplateGenConfig,
        typescript::type_map_service::TypescriptTypeMapService,
    },
    response::File,
};

mod type_map_service;

pub fn postgres() -> TemplateGenConfig {
    TemplateGenConfig {
        query_directory_entrypoint: "queries.ts",
        model_directory_entrypoint: "models.ts",
        file_extension: "ts",
        query_template: include_str!("./templates/postgres/query.j2"),
        model_template: include_str!("./templates/postgres/model.j2"),
        model_init_template: include_str!("./templates/postgres/model_init.j2"),
        type_map_service: &TypescriptTypeMapService,
        other_templates: vec![File {
            path: "parsers.ts".into(),
            content: include_str!("./templates/parsers.ts").into(),
        }],
        register_filters: Some(register_filters),
    }
}

fn register_filters(env: &mut Environment) -> Result<(), Error> {
    env.add_filter("is_nullable", move |ty: Value| -> bool {
        matches!(Type::from_jinja(ty), Type::Nullable(_))
    });

    env.add_filter("type_parser", |value: Value| {
        TypescriptTypeMapService.type_parser(Type::from_jinja(value))
    });

    env.add_filter("is_user_defined", |value: Value| -> bool {
        matches!(Type::from_jinja(value), Type::UserDefined { .. })
    });

    env.add_filter("requires_parsing", |value: Value| -> bool {
        TypescriptTypeMapService.column_requires_parser(Type::from_jinja(value))
    });

    Ok(())
}
