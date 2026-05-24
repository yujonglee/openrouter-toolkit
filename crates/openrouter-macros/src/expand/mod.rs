use proc_macro::TokenStream;
use std::collections::HashSet;

use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, LitStr, Path};

use crate::input::ModelSupportsInput;
use openrouter_models::{
    known_capability_names, missing_model_capabilities, ModelLookupError, Namespace,
};

pub(crate) fn model_supports(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ModelSupportsInput);
    let model_id = input.model.value();

    let required_capabilities = match parse_capabilities(&input.required_capabilities) {
        Ok(required_capabilities) => required_capabilities,
        Err(error) => return error.to_compile_error().into(),
    };
    let required_capability_refs: Vec<(Namespace, &str)> = required_capabilities
        .iter()
        .map(|capability| (capability.namespace, capability.name.as_str()))
        .collect();

    match missing_model_capabilities(&model_id, &required_capability_refs) {
        Ok(missing) if missing.is_empty() => {
            let model = input.model;
            quote!(#model).into()
        }
        Ok(missing) => compile_error(
            &input.model,
            format_args!(
                "OpenRouter model `{model_id}` does not support required capability(s): {}",
                format_capabilities(
                    missing
                        .iter()
                        .map(|(namespace, name)| (*namespace, name.as_str()))
                )
            ),
        ),
        Err(ModelLookupError::UnknownModel) => compile_error(
            &input.model,
            format_args!("unknown OpenRouter model `{model_id}`"),
        ),
        Err(ModelLookupError::InvalidModelsJson(message)) => compile_error(
            &input.model,
            format_args!("invalid embedded OpenRouter models JSON: {message}"),
        ),
    }
}

#[derive(Debug)]
struct RequiredCapability {
    namespace: Namespace,
    name: String,
}

fn parse_capabilities(
    capabilities: &syn::punctuated::Punctuated<Path, syn::Token![,]>,
) -> syn::Result<Vec<RequiredCapability>> {
    let mut parsed = Vec::new();
    let mut seen = HashSet::new();

    for capability in capabilities {
        let (namespace, name) = parse_capability_path(capability)?;
        let key = (namespace, name.clone());
        if !seen.insert(key) {
            return Err(syn::Error::new(
                capability.span(),
                format!(
                    "duplicate OpenRouter capability `{}`",
                    format_capability(namespace, &name)
                ),
            ));
        }

        parsed.push(RequiredCapability { namespace, name });
    }

    Ok(parsed)
}

fn parse_capability_path(path: &Path) -> syn::Result<(Namespace, String)> {
    if path.leading_colon.is_some() || path.segments.len() != 2 {
        return Err(syn::Error::new(
            path.span(),
            "expected OpenRouter capability path in the form `param::name`, `input::name`, or `output::name`",
        ));
    }

    let mut segments = path.segments.iter();
    let namespace_segment = segments
        .next()
        .expect("two-segment path should have a namespace");
    let name_segment = segments
        .next()
        .expect("two-segment path should have a name");

    if !namespace_segment.arguments.is_empty() || !name_segment.arguments.is_empty() {
        return Err(syn::Error::new(
            path.span(),
            "OpenRouter capability paths cannot contain generic arguments",
        ));
    }

    let namespace = match namespace_segment.ident.to_string().as_str() {
        "param" => Namespace::Param,
        "input" => Namespace::Input,
        "output" => Namespace::Output,
        unknown => {
            return Err(syn::Error::new(
                namespace_segment.ident.span(),
                format!(
                    "unknown OpenRouter capability namespace `{unknown}`; expected one of: param, input, output"
                ),
            ));
        }
    };
    let name = name_segment.ident.to_string();

    let known_names = known_capability_names(namespace).map_err(|err| match err {
        ModelLookupError::InvalidModelsJson(message) => syn::Error::new(
            name_segment.ident.span(),
            format!("invalid embedded OpenRouter models JSON: {message}"),
        ),
        ModelLookupError::UnknownModel => syn::Error::new(
            name_segment.ident.span(),
            "failed to load OpenRouter capability names",
        ),
    })?;

    if !known_names.contains(&name) {
        let suggestion = closest_name(&name, known_names)
            .map(|known_name| {
                format!(
                    "; did you mean `{}`?",
                    format_capability(namespace, known_name)
                )
            })
            .unwrap_or_else(|| {
                format!(
                    "; known {} capabilities: {}",
                    namespace.as_str(),
                    sorted_names(known_names).join(", ")
                )
            });

        return Err(syn::Error::new(
            name_segment.ident.span(),
            format!(
                "unknown OpenRouter capability `{}`{}",
                format_capability(namespace, &name),
                suggestion,
            ),
        ));
    }

    Ok((namespace, name))
}

fn format_capabilities<'a>(capabilities: impl Iterator<Item = (Namespace, &'a str)>) -> String {
    capabilities
        .map(|(namespace, name)| format_capability(namespace, name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_capability(namespace: Namespace, name: &str) -> String {
    format!("{}::{name}", namespace.as_str())
}

fn sorted_names(names: &HashSet<String>) -> Vec<&str> {
    let mut names: Vec<_> = names.iter().map(String::as_str).collect();
    names.sort_unstable();
    names
}

fn closest_name<'a>(name: &str, known_names: &'a HashSet<String>) -> Option<&'a str> {
    known_names
        .iter()
        .map(|known_name| (known_name.as_str(), edit_distance(name, known_name)))
        .filter(|(_, distance)| *distance <= 3)
        .min_by_key(|(known_name, distance)| (*distance, known_name.len()))
        .map(|(known_name, _)| known_name)
}

fn edit_distance(left: &str, right: &str) -> usize {
    let right_chars: Vec<_> = right.chars().collect();
    let mut previous: Vec<_> = (0..=right_chars.len()).collect();
    let mut current = vec![0; right_chars.len() + 1];

    for (left_index, left_char) in left.chars().enumerate() {
        current[0] = left_index + 1;

        for (right_index, right_char) in right_chars.iter().enumerate() {
            let insertion = current[right_index] + 1;
            let deletion = previous[right_index + 1] + 1;
            let substitution = previous[right_index] + usize::from(left_char != *right_char);
            current[right_index + 1] = insertion.min(deletion).min(substitution);
        }

        std::mem::swap(&mut previous, &mut current);
    }

    previous[right_chars.len()]
}

fn compile_error(span: &LitStr, message: impl std::fmt::Display) -> TokenStream {
    compile_error_at(span.span(), message)
}

fn compile_error_at(span: Span, message: impl std::fmt::Display) -> TokenStream {
    let error = syn::Error::new(span, message);
    error.to_compile_error().into()
}

#[cfg(test)]
mod tests;
