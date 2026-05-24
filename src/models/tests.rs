use rstest::rstest;

use crate::test_support::{synthetic_index, DUPLICATE_MODELS_JSON};

use super::{normalize_model_id_for_lookup, ModelIndex, ModelLookupError, Namespace};

fn owned_capabilities(capabilities: &[(Namespace, &str)]) -> Vec<(Namespace, String)> {
    capabilities
        .iter()
        .map(|(namespace, name)| (*namespace, (*name).to_owned()))
        .collect()
}

#[rstest]
#[case::single_param("model/full", &[(Namespace::Param, "tools")], &[])]
#[case::exacto_variant("model/full:exacto", &[(Namespace::Param, "tools")], &[])]
#[case::nitro_variant("model/full:nitro", &[(Namespace::Param, "tools")], &[])]
#[case::floor_variant("model/full:floor", &[(Namespace::Param, "tools")], &[])]
#[case::online_variant("model/full:online", &[(Namespace::Input, "image")], &[])]
#[case::all_namespaces("model/full", &[(Namespace::Param, "tools"), (Namespace::Param, "response_format"), (Namespace::Input, "image"), (Namespace::Output, "text")], &[])]
#[case::missing_param("model/full", &[(Namespace::Param, "seed")], &[(Namespace::Param, "seed")])]
#[case::missing_input_variant("model/full:exacto", &[(Namespace::Input, "audio")], &[(Namespace::Input, "audio")])]
#[case::mixed_missing_preserves_order("model/full", &[(Namespace::Param, "seed"), (Namespace::Param, "tools"), (Namespace::Output, "audio")], &[(Namespace::Param, "seed"), (Namespace::Output, "audio")])]
#[case::empty_model("model/empty", &[(Namespace::Param, "tools")], &[(Namespace::Param, "tools")])]
#[case::null_fields("model/null", &[(Namespace::Input, "image")], &[(Namespace::Input, "image")])]
#[case::missing_architecture("model/absent", &[(Namespace::Output, "text")], &[(Namespace::Output, "text")])]
#[case::missing_modality_arrays("model/missing_modalities", &[(Namespace::Param, "tools"), (Namespace::Input, "text")], &[(Namespace::Input, "text")])]
fn reports_missing_capabilities(
    synthetic_index: &ModelIndex,
    #[case] model_id: &str,
    #[case] required_capabilities: &[(Namespace, &str)],
    #[case] expected_missing: &[(Namespace, &str)],
) {
    assert_eq!(
        synthetic_index.missing_capabilities(model_id, required_capabilities),
        Ok(owned_capabilities(expected_missing)),
    );
}

#[rstest]
#[case::unknown_base("model/unknown")]
#[case::unknown_dynamic_variant("model/unknown:exacto")]
#[case::static_variant_is_not_stripped("model/full:free")]
fn reports_unknown_model(synthetic_index: &ModelIndex, #[case] model_id: &str) {
    assert_eq!(
        synthetic_index.missing_capabilities(model_id, &[(Namespace::Param, "tools")]),
        Err(ModelLookupError::UnknownModel),
    );
}

#[rstest]
#[case::param(Namespace::Param, &["response_format", "structured_outputs", "tools"])]
#[case::input(Namespace::Input, &["image", "text"])]
#[case::output(Namespace::Output, &["image", "text"])]
fn tracks_known_capability_names(
    synthetic_index: &ModelIndex,
    #[case] namespace: Namespace,
    #[case] expected_names: &[&str],
) {
    for expected_name in expected_names {
        assert!(
            synthetic_index
                .known_names(namespace)
                .contains(*expected_name),
            "expected {namespace:?} to include {expected_name}",
        );
    }
}

#[rstest]
#[case::bare("model/full", "model/full")]
#[case::exacto("model/full:exacto", "model/full")]
#[case::nitro("model/full:nitro", "model/full")]
#[case::floor("model/full:floor", "model/full")]
#[case::online("model/full:online", "model/full")]
#[case::static_variant("model/full:free", "model/full:free")]
#[case::suffix_only(":nitro", "")]
#[case::multi_suffix("model/full:exacto:nitro", "model/full:exacto")]
fn normalizes_dynamic_variants(#[case] model_id: &str, #[case] expected: &str) {
    assert_eq!(normalize_model_id_for_lookup(model_id), expected);
}

#[rstest]
#[case::invalid_json("{")]
#[case::missing_data(r#"{"models":[]}"#)]
#[case::null_data(r#"{"data":null}"#)]
#[case::non_string_id(r#"{"data":[{"id":1}]}"#)]
fn reports_invalid_json_shapes(#[case] json: &str) {
    let error = ModelIndex::parse_json(json).expect_err("invalid shape should fail");

    assert!(matches!(error, ModelLookupError::InvalidModelsJson(_)));
}

#[test]
fn duplicate_model_ids_use_last_entry() {
    let index =
        ModelIndex::parse_json(DUPLICATE_MODELS_JSON).expect("duplicate fixture should parse");

    assert_eq!(
        index.missing_capabilities(
            "model/duplicate",
            &[(Namespace::Param, "tools"), (Namespace::Param, "seed")],
        ),
        Ok(owned_capabilities(&[(Namespace::Param, "tools")])),
    );
}
