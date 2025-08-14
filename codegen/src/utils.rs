use std::{
    collections::{BTreeMap, HashMap},
    fmt::format,
    sync::{LazyLock, Mutex},
};

use heck::{ToKebabCase, ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use minijinja::{Environment, Template};
use regex::bytes::Regex;
use serde::Serialize;

pub fn env() -> Environment<'static> {
    let mut env = minijinja::Environment::new();

    env.add_filter("to_camel_case", to_camel_case);
    env.add_filter("to_pascal_case", to_pascal_case);
    env.add_filter("to_snake_case", to_snake_case);
    env.add_filter("to_kebab_case", to_kebab_case);
    env.add_filter("to_screaming_snake_case", to_screaming_snake_case);
    env.add_filter("to_c_string", to_c_string);
    env.add_filter("starts_with", starts_with);
    env.add_filter("strip_prefix", strip_prefix);
    env.add_filter("regex_replace", regex_replace);
    env
}

pub fn render<T: Serialize>(template: &str, context: T) -> String {
    env().render_named_str("root", template, context).unwrap()
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
