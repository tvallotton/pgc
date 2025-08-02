use std::fmt::format;

use heck::{ToKebabCase, ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use minijinja::{Environment, Template};
use serde::Serialize;

pub fn env() -> Environment<'static> {
    let mut env = minijinja::Environment::new();

    env.add_filter("to_camel_case", to_camel_case);
    env.add_filter("to_pascal_case", to_pascal_case);
    env.add_filter("to_snake_case", to_snake_case);
    env.add_filter("to_kebab_case", to_kebab_case);
    env.add_filter("to_screaming_snake_case", to_screaming_snake_case);
    env.add_filter("to_c_string", to_c_string);
    env
}

pub fn render<T: Serialize>(template: &str, context: T) -> String {
    env().render_named_str("root", template, context).unwrap()
}
pub fn to_c_string(s: &str) -> String {
    format!("{:?}", s)
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
