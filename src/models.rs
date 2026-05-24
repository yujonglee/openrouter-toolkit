use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use serde::Deserialize;

const MODELS_JSON: &str = include_str!("../data/models.json");

static MODELS: LazyLock<Result<ModelIndex, ModelLookupError>> = LazyLock::new(ModelIndex::parse);

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<Model>,
}

#[derive(Deserialize)]
struct Model {
    id: String,
    supported_parameters: Option<Vec<String>>,
}

#[derive(Debug)]
struct ModelIndex {
    models: HashMap<String, HashSet<String>>,
}

impl ModelIndex {
    fn parse() -> Result<Self, ModelLookupError> {
        Self::parse_json(MODELS_JSON)
    }

    fn parse_json(json: &str) -> Result<Self, ModelLookupError> {
        let models: ModelsResponse = serde_json::from_str(json)
            .map_err(|err| ModelLookupError::InvalidModelsJson(err.to_string()))?;

        Ok(Self {
            models: models
                .data
                .into_iter()
                .map(|model| {
                    let supported_parameters = model
                        .supported_parameters
                        .unwrap_or_default()
                        .into_iter()
                        .collect();

                    (model.id, supported_parameters)
                })
                .collect(),
        })
    }

    fn supports_parameters(
        &self,
        model_id: &str,
        required_parameters: &[&str],
    ) -> Result<Vec<String>, ModelLookupError> {
        let supported_parameters = self
            .models
            .get(model_id)
            .ok_or(ModelLookupError::UnknownModel)?;

        Ok(required_parameters
            .iter()
            .filter(|required_parameter| !supported_parameters.contains(**required_parameter))
            .map(|required_parameter| (*required_parameter).to_owned())
            .collect())
    }
}

pub(crate) fn model_supports_parameter(
    model_id: &str,
    required_parameter: &'static str,
) -> Result<bool, ModelLookupError> {
    Ok(missing_model_parameters(model_id, &[required_parameter])?.is_empty())
}

pub(crate) fn missing_model_parameters(
    model_id: &str,
    required_parameters: &[&str],
) -> Result<Vec<String>, ModelLookupError> {
    match &*MODELS {
        Ok(models) => models.supports_parameters(model_id, required_parameters),
        Err(err) => Err(err.clone()),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ModelLookupError {
    UnknownModel,
    InvalidModelsJson(String),
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{ModelIndex, ModelLookupError};

    const MODELS_JSON: &str = r#"
        {
          "data": [
            {
              "id": "model/full",
              "supported_parameters": ["structured_outputs", "tools", "response_format"]
            },
            {
              "id": "model/empty",
              "supported_parameters": []
            },
            {
              "id": "model/null",
              "supported_parameters": null
            },
            {
              "id": "model/absent"
            }
          ]
        }
    "#;

    #[rstest]
    #[case("model/full", &["tools"], &[])]
    #[case("model/full", &["tools", "response_format"], &[])]
    #[case("model/full", &["seed"], &["seed"])]
    #[case("model/full", &["seed", "tools", "top_p"], &["seed", "top_p"])]
    #[case("model/empty", &["tools"], &["tools"])]
    #[case("model/null", &["tools"], &["tools"])]
    #[case("model/absent", &["tools"], &["tools"])]
    fn reports_missing_parameters(
        #[case] model_id: &str,
        #[case] required_parameters: &[&str],
        #[case] expected_missing: &[&str],
    ) {
        let index = ModelIndex::parse_json(MODELS_JSON).expect("test models should parse");

        assert_eq!(
            index.supports_parameters(model_id, required_parameters),
            Ok(expected_missing
                .iter()
                .map(|parameter| (*parameter).to_owned())
                .collect()),
        );
    }

    #[test]
    fn reports_unknown_model() {
        let index = ModelIndex::parse_json(MODELS_JSON).expect("test models should parse");

        assert_eq!(
            index.supports_parameters("model/unknown", &["tools"]),
            Err(ModelLookupError::UnknownModel),
        );
    }

    #[test]
    fn reports_invalid_json() {
        let error = ModelIndex::parse_json("{").expect_err("invalid JSON should fail");

        assert!(matches!(error, ModelLookupError::InvalidModelsJson(_)));
    }
}
