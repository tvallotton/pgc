use std::{collections::BTreeMap, sync::Mutex};

use heck::{ToKebabCase, ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use minijinja::Environment;
use regex::bytes::Regex;
use serde::Serialize;

pub fn to_camel_case(s: &str) -> String {
    s.to_lower_camel_case()
}

pub fn to_pascal_case(s: &str) -> String {
    s.to_upper_camel_case()
}
