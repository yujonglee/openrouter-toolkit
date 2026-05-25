use openrouter_models::{ModelLookupError, Namespace};

use crate::format::format_capability;

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("{message}")]
    InvalidCapabilityPath { message: String },
    #[error("unknown OpenRouter capability namespace `{unknown}`; expected one of: param, input, output")]
    UnknownNamespace { unknown: String },
    #[error("{}", format_model_lookup_error(.0))]
    ModelLookup(ModelLookupError),
    #[error("unknown OpenRouter capability `{}`{suggestion}", format_capability(*namespace, name))]
    UnknownCapability {
        namespace: Namespace,
        name: String,
        suggestion: String,
    },
}

impl Error {
    pub(crate) fn invalid_capability_path(message: impl Into<String>) -> Self {
        Self::InvalidCapabilityPath {
            message: message.into(),
        }
    }

    pub(crate) fn unknown_namespace(unknown: impl Into<String>) -> Self {
        Self::UnknownNamespace {
            unknown: unknown.into(),
        }
    }

    pub(crate) fn unknown_capability(
        namespace: Namespace,
        name: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self::UnknownCapability {
            namespace,
            name: name.into(),
            suggestion: suggestion.into(),
        }
    }
}

fn format_model_lookup_error(error: &ModelLookupError) -> String {
    match error {
        ModelLookupError::InvalidModelsJson(message) => {
            format!("invalid embedded OpenRouter models JSON: {message}")
        }
        ModelLookupError::UnknownModel => "failed to load OpenRouter capability names".to_owned(),
    }
}
