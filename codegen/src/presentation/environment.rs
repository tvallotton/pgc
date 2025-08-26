use std::{
    collections::BTreeMap,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{Arc, Mutex},
};

use heck::{ToKebabCase, ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use indexmap::map::serde_seq::deserialize;
use minijinja::{Environment, State, Value, context, value::Object};
use regex::bytes::Regex;
use serde::{Deserialize, Deserializer, Serialize, de::IntoDeserializer};

use crate::{
    error::Error,
    ir::{Ir, Type},
    presentation::{
        file_generation_config::TemplateGenConfig,
        type_mapping_service::{OverriddenTypeMapService, TypeMapService},
    },
};

pub fn env(ir: Ir, config: TemplateGenConfig) -> Result<Environment<'static>, Error> {
    let mut env = minijinja::Environment::new();

    add_templates(&mut env, config)?;
    add_string_filters(&mut env);
    add_type_filters(&mut env, ir, config);

    Ok(env)
}

pub fn add_type_filters(env: &mut Environment<'static>, ir: Ir, config: TemplateGenConfig) {
    let service = Arc::new(OverriddenTypeMapService::new(ir, config.type_map_service));

    let service_ = service.clone();

    env.add_filter("annotation", move |state: &State, ty: Value| -> Arc<str> {
        service_.get(module_path(state), &as_type(ty)).annotation
    });

    let service_ = service.clone();

    env.add_filter(
        "name",
        move |state: &State, ty: Value| -> Option<Arc<str>> {
            service_.get(module_path(state), &as_type(ty)).name
        },
    );
    let service_ = service.clone();
    env.add_filter(
        "imports",
        move |state: &State, ty: Value| -> Vec<Arc<str>> {
            service_.get(module_path(state), &as_type(ty)).import
        },
    );
    let service_ = service.clone();
    env.add_filter(
        "type_module",
        move |state: &State, ty: Value| -> Option<Arc<str>> {
            service_.get(module_path(state), &as_type(ty)).module
        },
    );
}

pub fn add_templates(
    env: &mut Environment<'static>,
    config: TemplateGenConfig,
) -> Result<(), Error> {
    env.add_template("query", config.query_template)?;
    env.add_template("model", config.model_template)?;
    env.add_template("model_init", config.model_init_template)?;
    Ok(())
}

pub fn add_string_filters(env: &mut Environment<'static>) {
    env.add_filter("to_camel_case", to_camel_case);
    env.add_filter("to_pascal_case", to_pascal_case);
    env.add_filter("to_snake_case", to_snake_case);
    env.add_filter("to_kebab_case", to_kebab_case);
    env.add_filter("to_screaming_snake_case", to_screaming_snake_case);
    env.add_filter("to_c_string", to_c_string);
    env.add_filter("starts_with", starts_with);
    env.add_filter("strip_prefix", strip_prefix);
    env.add_filter("regex_replace", regex_replace);
}

pub fn module_path<'a>(state: &State<'_, 'a>) -> Vec<String> {
    use serde::de::value::SeqDeserializer;
    let this_module = state.lookup("this_module").unwrap();
    return <Vec<String>>::deserialize(SeqDeserializer::new(this_module.try_iter().unwrap()))
        .unwrap();
}

pub fn as_type(value: Value) -> Type {
    let Ok(value) = serde_json::to_value(value) else {
        return Type::AnyEnum;
    };
    match serde_json::from_value(value) {
        Ok(ty) => ty,
        Err(err) => Type::Other {
            schema: format!("{err:?}").into(),
            name: "failed".into(),
        },
    }
}

pub fn regex_replace(text: &str, pattern: &str, replacement: &str) -> String {
    static REGEXES: Mutex<BTreeMap<String, Regex>> = Mutex::new(BTreeMap::new());
    let mut guard = REGEXES.lock().unwrap();
    let entry = guard.entry(pattern.into());
    let regex = entry.or_insert_with(|| Regex::new(pattern).unwrap());
    String::from_utf8(
        regex
            .replace_all(text.as_bytes(), replacement.as_bytes())
            .into(),
    )
    .unwrap()
}

pub fn to_c_string(s: &str) -> String {
    format!("{:?}", s)
}

pub fn strip_prefix<'a>(text: &'a str, pattern: &str) -> String {
    text.strip_prefix(pattern).unwrap_or(text).to_string()
}

pub fn starts_with(text: &str, pattern: &str) -> bool {
    text.starts_with(pattern)
}

pub fn to_camel_case(s: &str) -> String {
    s.to_lower_camel_case()
}

pub fn to_pascal_case(s: &str) -> String {
    s.to_upper_camel_case()
}

pub fn to_snake_case(s: &str) -> String {
    s.to_snake_case()
}

pub fn to_screaming_snake_case(s: &str) -> String {
    s.to_shouty_snake_case()
}

pub fn to_kebab_case(s: &str) -> String {
    s.to_kebab_case()
}

#[test]
fn foo() {
    let mut env = Environment::new();

    env.add_filter("foo", |ty: &Value| {
        let ty: Type = serde_json::from_value(serde_json::to_value(ty).unwrap()).unwrap();
        "works"
    });

    let content = env.render_str("{{ x | foo }}", context! { x => Type::MacAddr});
    assert!(content.unwrap().contains("works"));
}
