use std::collections::HashSet;

use openrouter_models::Namespace;

pub fn format_capabilities<'a>(
    capabilities: impl IntoIterator<Item = (Namespace, &'a str)>,
) -> String {
    capabilities
        .into_iter()
        .map(|(namespace, name)| format_capability(namespace, name))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn format_capability(namespace: Namespace, name: &str) -> String {
    format!("{}::{name}", namespace.as_str())
}

pub fn sorted_names(names: &HashSet<String>) -> Vec<&str> {
    let mut names: Vec<_> = names.iter().map(String::as_str).collect();
    names.sort_unstable();
    names
}
