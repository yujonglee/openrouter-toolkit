use std::collections::HashSet;

use rstest::fixture;

use crate::models::ModelIndex;

pub(crate) const SYNTHETIC_MODELS_JSON: &str = r#"
{
  "data": [
    {
      "id": "model/full",
      "supported_parameters": ["structured_outputs", "tools", "response_format"],
      "architecture": {
        "input_modalities": ["text", "image"],
        "output_modalities": ["text", "image"]
      }
    },
    {
      "id": "model/empty",
      "supported_parameters": [],
      "architecture": {
        "input_modalities": [],
        "output_modalities": []
      }
    },
    {
      "id": "model/null",
      "supported_parameters": null,
      "architecture": {
        "input_modalities": null,
        "output_modalities": null
      }
    },
    {
      "id": "model/missing_modalities",
      "supported_parameters": ["tools"],
      "architecture": {}
    },
    {
      "id": "model/absent"
    }
  ]
}
"#;

pub(crate) const DUPLICATE_MODELS_JSON: &str = r#"
{
  "data": [
    {
      "id": "model/duplicate",
      "supported_parameters": ["tools"],
      "architecture": { "input_modalities": ["text"], "output_modalities": ["text"] }
    },
    {
      "id": "model/duplicate",
      "supported_parameters": ["seed"],
      "architecture": { "input_modalities": ["image"], "output_modalities": ["image"] }
    }
  ]
}
"#;

#[fixture]
#[once]
pub(crate) fn synthetic_index() -> ModelIndex {
    ModelIndex::parse_json(SYNTHETIC_MODELS_JSON).expect("synthetic models should parse")
}

#[fixture]
pub(crate) fn known_param_names() -> HashSet<String> {
    [
        "tools",
        "tool",
        "response_format",
        "structured_outputs",
        "seed",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
