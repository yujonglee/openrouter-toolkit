use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use crate::input::ModelSupportsInput;
use crate::models::{missing_model_parameters, model_supports_parameter, ModelLookupError};

pub(crate) fn model_supports(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ModelSupportsInput);
    let model_id = input.model.value();
    let required_parameters: Vec<String> = input
        .required_parameters
        .iter()
        .map(LitStr::value)
        .collect();
    let required_parameter_refs: Vec<&str> =
        required_parameters.iter().map(String::as_str).collect();

    match missing_model_parameters(&model_id, &required_parameter_refs) {
        Ok(missing) if missing.is_empty() => {
            let model = input.model;
            quote!(#model).into()
        }
        Ok(missing) => compile_error(
            &input.model,
            format_args!(
                "OpenRouter model `{model_id}` does not support required parameter(s): {}",
                missing.join(", ")
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

pub(crate) fn model_parameter(input: TokenStream, required_parameter: &'static str) -> TokenStream {
    let model = parse_macro_input!(input as LitStr);
    let model_id = model.value();

    match model_supports_parameter(&model_id, required_parameter) {
        Ok(true) => quote!(#model).into(),
        Ok(false) => compile_error(
            &model,
            format_args!("OpenRouter model `{model_id}` does not support `{required_parameter}`"),
        ),
        Err(ModelLookupError::UnknownModel) => compile_error(
            &model,
            format_args!("unknown OpenRouter model `{model_id}`"),
        ),
        Err(ModelLookupError::InvalidModelsJson(message)) => compile_error(
            &model,
            format_args!("invalid embedded OpenRouter models JSON: {message}"),
        ),
    }
}

fn compile_error(span: &LitStr, message: impl std::fmt::Display) -> TokenStream {
    let error = syn::Error::new(span.span(), message);
    error.to_compile_error().into()
}
