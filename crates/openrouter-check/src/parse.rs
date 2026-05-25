use std::fmt;

use openrouter_models::{known_capability_names, ModelLookupError, Namespace};

use crate::format::{format_capability, sorted_names};
use crate::suggest::closest_name;

pub const CAPABILITY_PATH_MESSAGE: &str =
    "expected OpenRouter capability path in the form `param::name`, `input::name`, or `output::name`";

pub fn parse_capability(path: &str) -> Result<(Namespace, String), InvalidCapabilityPath> {
    let mut segments = path.split("::");
    let Some(namespace) = segments.next() else {
        return Err(InvalidCapabilityPath::new());
    };
    let Some(name) = segments.next() else {
        return Err(InvalidCapabilityPath::new());
    };
    if namespace.is_empty() || name.is_empty() || segments.next().is_some() {
        return Err(InvalidCapabilityPath::new());
    }

    Ok((
        parse_namespace(namespace).map_err(|error| InvalidCapabilityPath {
            message: error.to_string(),
        })?,
        name.to_owned(),
    ))
}

pub fn parse_namespace(text: &str) -> Result<Namespace, UnknownNamespace> {
    match text {
        "param" => Ok(Namespace::Param),
        "input" => Ok(Namespace::Input),
        "output" => Ok(Namespace::Output),
        unknown => Err(UnknownNamespace {
            unknown: unknown.to_owned(),
        }),
    }
}

pub fn validate_capability_name(
    namespace: Namespace,
    name: &str,
) -> Result<(), CapabilityNameError> {
    let known_names = known_capability_names(namespace).map_err(CapabilityNameError::Lookup)?;

    if !known_names.contains(name) {
        return Err(CapabilityNameError::Unknown(UnknownCapability::new(
            namespace,
            name,
            known_names,
        )));
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidCapabilityPath {
    pub message: String,
}

impl InvalidCapabilityPath {
    fn new() -> Self {
        Self {
            message: CAPABILITY_PATH_MESSAGE.to_owned(),
        }
    }
}

impl fmt::Display for InvalidCapabilityPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for InvalidCapabilityPath {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownNamespace {
    unknown: String,
}

impl fmt::Display for UnknownNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown OpenRouter capability namespace `{}`; expected one of: param, input, output",
            self.unknown
        )
    }
}

impl std::error::Error for UnknownNamespace {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapabilityNameError {
    Lookup(ModelLookupError),
    Unknown(UnknownCapability),
}

impl fmt::Display for CapabilityNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lookup(ModelLookupError::InvalidModelsJson(message)) => {
                write!(
                    formatter,
                    "invalid embedded OpenRouter models JSON: {message}"
                )
            }
            Self::Lookup(ModelLookupError::UnknownModel) => {
                formatter.write_str("failed to load OpenRouter capability names")
            }
            Self::Unknown(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for CapabilityNameError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownCapability {
    namespace: Namespace,
    name: String,
    suggestion: UnknownCapabilitySuggestion,
}

impl UnknownCapability {
    fn new(
        namespace: Namespace,
        name: &str,
        known_names: &std::collections::HashSet<String>,
    ) -> Self {
        let suggestion = closest_name(name, known_names)
            .map(|known_name| UnknownCapabilitySuggestion::Closest {
                name: known_name.to_owned(),
            })
            .unwrap_or_else(|| UnknownCapabilitySuggestion::Known {
                names: sorted_names(known_names)
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
            });

        Self {
            namespace,
            name: name.to_owned(),
            suggestion,
        }
    }
}

impl fmt::Display for UnknownCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown OpenRouter capability `{}`",
            format_capability(self.namespace, &self.name)
        )?;

        match &self.suggestion {
            UnknownCapabilitySuggestion::Closest { name } => {
                write!(
                    formatter,
                    "; did you mean `{}`?",
                    format_capability(self.namespace, name)
                )
            }
            UnknownCapabilitySuggestion::Known { names } => {
                write!(
                    formatter,
                    "; known {} capabilities: {}",
                    self.namespace.as_str(),
                    names.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for UnknownCapability {}

#[derive(Clone, Debug, PartialEq, Eq)]
enum UnknownCapabilitySuggestion {
    Closest { name: String },
    Known { names: Vec<String> },
}
