use proc_macro::TokenStream;
use std::collections::HashSet;

use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, LitStr, Path};

use crate::input::ModelSupportsInput;
use openrouter_check::{
    format_capabilities, format_capability, missing_model_capabilities, parse_namespace,
    validate_capability_name, ModelLookupError, Namespace,
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

    let namespace_text = namespace_segment.ident.to_string();
    let namespace = parse_namespace(&namespace_text)
        .map_err(|error| syn::Error::new(namespace_segment.ident.span(), error.to_string()))?;
    let name = name_segment.ident.to_string();

    validate_capability_name(namespace, &name)
        .map_err(|error| syn::Error::new(name_segment.ident.span(), error.to_string()))?;

    Ok((namespace, name))
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
