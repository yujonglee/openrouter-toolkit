mod error;
mod format;
mod parse;
mod suggest;

use std::collections::HashSet;

pub use crate::error::Error;
pub use crate::format::{format_capabilities, format_capability, sorted_names};
pub use crate::parse::{parse_capability, parse_namespace, validate_capability_name};
pub use openrouter_models::{missing_model_capabilities, ModelLookupError, Namespace};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub kind: DiagnosticKind,
    pub source_index: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    DuplicateCapability,
    InvalidCapabilityPath,
    InvalidModelsJson,
    MissingCapability,
    UnknownCapability,
    UnknownModel,
    UnknownNamespace,
}

pub fn check(model: &str, capabilities: &[&str]) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut required_capabilities = Vec::new();
    let mut seen = HashSet::new();

    for (source_index, capability) in capabilities.iter().enumerate() {
        let (namespace, name) = match parse_capability(capability) {
            Ok(capability) => capability,
            Err(error) => {
                diagnostics.push(Diagnostic {
                    message: error.to_string(),
                    kind: diagnostic_kind(&error),
                    source_index: Some(source_index),
                });
                continue;
            }
        };

        if let Err(error) = validate_capability_name(namespace, &name) {
            diagnostics.push(Diagnostic {
                message: error.to_string(),
                kind: diagnostic_kind(&error),
                source_index: Some(source_index),
            });
            continue;
        }

        if !seen.insert((namespace, name.clone())) {
            diagnostics.push(Diagnostic {
                message: format!(
                    "duplicate OpenRouter capability `{}`",
                    format_capability(namespace, &name)
                ),
                kind: DiagnosticKind::DuplicateCapability,
                source_index: Some(source_index),
            });
            continue;
        }

        required_capabilities.push((namespace, name));
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    let required_capability_refs: Vec<_> = required_capabilities
        .iter()
        .map(|(namespace, name)| (*namespace, name.as_str()))
        .collect();

    match missing_model_capabilities(model, &required_capability_refs) {
        Ok(missing) if missing.is_empty() => Ok(()),
        Ok(missing) => Err(vec![Diagnostic {
            message: format!(
                "OpenRouter model `{model}` does not support required capability(s): {}",
                format_capabilities(
                    missing
                        .iter()
                        .map(|(namespace, name)| (*namespace, name.as_str()))
                )
            ),
            kind: DiagnosticKind::MissingCapability,
            source_index: None,
        }]),
        Err(ModelLookupError::UnknownModel) => Err(vec![Diagnostic {
            message: format!("unknown OpenRouter model `{model}`"),
            kind: DiagnosticKind::UnknownModel,
            source_index: None,
        }]),
        Err(ModelLookupError::InvalidModelsJson(message)) => Err(vec![Diagnostic {
            message: format!("invalid embedded OpenRouter models JSON: {message}"),
            kind: DiagnosticKind::InvalidModelsJson,
            source_index: None,
        }]),
    }
}

fn diagnostic_kind(error: &Error) -> DiagnosticKind {
    match error {
        Error::InvalidCapabilityPath { .. } => DiagnosticKind::InvalidCapabilityPath,
        Error::UnknownNamespace { .. } => DiagnosticKind::UnknownNamespace,
        Error::ModelLookup(ModelLookupError::InvalidModelsJson(_)) => {
            DiagnosticKind::InvalidModelsJson
        }
        Error::ModelLookup(ModelLookupError::UnknownModel) => DiagnosticKind::UnknownModel,
        Error::UnknownCapability { .. } => DiagnosticKind::UnknownCapability,
    }
}
