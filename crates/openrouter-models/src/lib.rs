use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use serde::Deserialize;

const MODELS_JSON: &str = include_str!("../data/models.json");
const DYNAMIC_VARIANTS: &[&str] = &[":exacto", ":nitro", ":floor", ":online"];

static MODELS: LazyLock<Result<ModelIndex, ModelLookupError>> = LazyLock::new(ModelIndex::parse);

#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct OpenRouterModelsResponse {
    data: Vec<OpenRouterModel>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct OpenRouterModel {
    id: String,
    supported_parameters: Option<Vec<String>>,
    architecture: Option<OpenRouterArchitecture>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct OpenRouterArchitecture {
    input_modalities: Option<Vec<String>>,
    output_modalities: Option<Vec<String>>,
}

#[derive(Debug)]
pub(crate) struct ModelIndex {
    models: HashMap<String, ModelCapabilities>,
    known_params: HashSet<String>,
    known_input_modalities: HashSet<String>,
    known_output_modalities: HashSet<String>,
}

#[derive(Debug)]
struct ModelCapabilities {
    params: HashSet<String>,
    input_modalities: HashSet<String>,
    output_modalities: HashSet<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Namespace {
    Param,
    Input,
    Output,
}

impl Namespace {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Param => "param",
            Self::Input => "input",
            Self::Output => "output",
        }
    }
}

impl ModelIndex {
    fn parse() -> Result<Self, ModelLookupError> {
        Self::parse_openrouter_json(MODELS_JSON)
    }

    pub(crate) fn parse_openrouter_json(json: &str) -> Result<Self, ModelLookupError> {
        let models: OpenRouterModelsResponse = serde_json::from_str(json)
            .map_err(|err| ModelLookupError::InvalidModelsJson(err.to_string()))?;

        validate_openrouter_models(&models).map_err(ModelLookupError::InvalidModelsJson)?;

        Ok(Self::from_openrouter_response(models))
    }

    #[cfg(test)]
    pub(crate) fn parse_synthetic_json(json: &str) -> Result<Self, ModelLookupError> {
        let models: OpenRouterModelsResponse = serde_json::from_str(json)
            .map_err(|err| ModelLookupError::InvalidModelsJson(err.to_string()))?;

        Ok(Self::from_openrouter_response(models))
    }

    fn from_openrouter_response(models: OpenRouterModelsResponse) -> Self {
        let mut known_params = HashSet::new();
        let mut known_input_modalities = HashSet::new();
        let mut known_output_modalities = HashSet::new();
        let mut indexed_models = HashMap::new();

        for model in models.data {
            let params: HashSet<_> = model
                .supported_parameters
                .unwrap_or_default()
                .into_iter()
                .collect();
            let architecture = model.architecture;
            let input_modalities: HashSet<_> = architecture
                .as_ref()
                .and_then(|architecture| architecture.input_modalities.clone())
                .unwrap_or_default()
                .into_iter()
                .collect();
            let output_modalities: HashSet<_> = architecture
                .and_then(|architecture| architecture.output_modalities)
                .unwrap_or_default()
                .into_iter()
                .collect();

            known_params.extend(params.iter().cloned());
            known_input_modalities.extend(input_modalities.iter().cloned());
            known_output_modalities.extend(output_modalities.iter().cloned());

            indexed_models.insert(
                model.id,
                ModelCapabilities {
                    params,
                    input_modalities,
                    output_modalities,
                },
            );
        }

        Self {
            models: indexed_models,
            known_params,
            known_input_modalities,
            known_output_modalities,
        }
    }

    pub(crate) fn missing_capabilities(
        &self,
        model_id: &str,
        required_capabilities: &[(Namespace, &str)],
    ) -> Result<Vec<(Namespace, String)>, ModelLookupError> {
        let lookup_model_id = normalize_model_id_for_lookup(model_id);
        let capabilities = self
            .models
            .get(lookup_model_id)
            .ok_or(ModelLookupError::UnknownModel)?;

        Ok(required_capabilities
            .iter()
            .filter(|(namespace, name)| !capabilities.contains(*namespace, name))
            .map(|(namespace, name)| (*namespace, (*name).to_owned()))
            .collect())
    }

    pub(crate) fn known_names(&self, namespace: Namespace) -> &HashSet<String> {
        match namespace {
            Namespace::Param => &self.known_params,
            Namespace::Input => &self.known_input_modalities,
            Namespace::Output => &self.known_output_modalities,
        }
    }
}

fn validate_openrouter_models(models: &OpenRouterModelsResponse) -> Result<(), String> {
    if models.data.is_empty() {
        return Err("OpenRouter models response must contain at least one model".to_owned());
    }

    let mut seen_ids = HashSet::new();
    for model in &models.data {
        if model.id.is_empty() {
            return Err("OpenRouter model ids must be non-empty".to_owned());
        }
        if !seen_ids.insert(model.id.as_str()) {
            return Err(format!("duplicate OpenRouter model id `{}`", model.id));
        }

        if let Some(parameters) = &model.supported_parameters {
            validate_non_empty_strings(parameters, "supported_parameters", &model.id)?;
        }

        if let Some(architecture) = &model.architecture {
            if let Some(input_modalities) = &architecture.input_modalities {
                validate_non_empty_strings(
                    input_modalities,
                    "architecture.input_modalities",
                    &model.id,
                )?;
            }
            if let Some(output_modalities) = &architecture.output_modalities {
                validate_non_empty_strings(
                    output_modalities,
                    "architecture.output_modalities",
                    &model.id,
                )?;
            }
        }
    }

    Ok(())
}

fn validate_non_empty_strings(
    values: &[String],
    field: &str,
    model_id: &str,
) -> Result<(), String> {
    if values.iter().any(String::is_empty) {
        return Err(format!(
            "OpenRouter model `{model_id}` contains an empty string in `{field}`"
        ));
    }

    Ok(())
}

impl ModelCapabilities {
    fn contains(&self, namespace: Namespace, name: &str) -> bool {
        match namespace {
            Namespace::Param => self.params.contains(name),
            Namespace::Input => self.input_modalities.contains(name),
            Namespace::Output => self.output_modalities.contains(name),
        }
    }
}

pub(crate) fn normalize_model_id_for_lookup(model_id: &str) -> &str {
    let mut lookup_model_id = model_id;

    while let Some(base_model_id) = DYNAMIC_VARIANTS
        .iter()
        .find_map(|variant| lookup_model_id.strip_suffix(variant))
    {
        if base_model_id == lookup_model_id {
            break;
        }

        lookup_model_id = base_model_id;
    }

    lookup_model_id
}

pub fn missing_model_capabilities(
    model_id: &str,
    required_capabilities: &[(Namespace, &str)],
) -> Result<Vec<(Namespace, String)>, ModelLookupError> {
    match &*MODELS {
        Ok(models) => models.missing_capabilities(model_id, required_capabilities),
        Err(err) => Err(err.clone()),
    }
}

pub fn known_capability_names(
    namespace: Namespace,
) -> Result<&'static HashSet<String>, ModelLookupError> {
    match &*MODELS {
        Ok(models) => Ok(models.known_names(namespace)),
        Err(err) => Err(err.clone()),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ModelLookupError {
    #[error("unknown OpenRouter model")]
    UnknownModel,
    #[error("invalid embedded OpenRouter models JSON: {0}")]
    InvalidModelsJson(String),
}

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
