use rstest::rstest;

#[rstest]
#[case::single("tests/ui/pass__single.rs")]
#[case::multiple("tests/ui/pass__multiple.rs")]
#[case::trailing_comma("tests/ui/pass__trailing_comma.rs")]
#[case::dynamic_variant("tests/ui/pass__dynamic_variant.rs")]
#[case::input_modality("tests/ui/pass__input_modality.rs")]
#[case::output_modality("tests/ui/pass__output_modality.rs")]
#[case::mixed_namespaces("tests/ui/pass__mixed_namespaces.rs")]
#[case::cross_namespace_same_name("tests/ui/pass__cross_namespace_same_name.rs")]
fn ui_pass(#[case] path: &str) {
    trybuild::TestCases::new().pass(path);
}

#[rstest]
#[case::no_capability("tests/ui/parse__no_capability.rs")]
#[case::missing_comma("tests/ui/parse__missing_comma.rs")]
#[case::non_literal_model("tests/ui/parse__non_literal_model.rs")]
#[case::path_wrong_arity("tests/ui/parse__path_wrong_arity.rs")]
#[case::leading_colon("tests/ui/parse__leading_colon.rs")]
#[case::generic_arguments("tests/ui/parse__generic_arguments.rs")]
fn ui_parse_errors(#[case] path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}

#[rstest]
#[case::unknown_namespace("tests/ui/capability__unknown_namespace.rs")]
#[case::unknown_name("tests/ui/capability__unknown_name.rs")]
#[case::unknown_name_no_suggestion("tests/ui/capability__unknown_name_no_suggestion.rs")]
#[case::duplicate("tests/ui/capability__duplicate.rs")]
fn ui_capability_errors(#[case] path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}

#[rstest]
#[case::unknown_model("tests/ui/lookup__unknown_model.rs")]
#[case::static_variant_unknown("tests/ui/lookup__static_variant_unknown.rs")]
#[case::param_missing("tests/ui/lookup__param_missing.rs")]
#[case::all_missing("tests/ui/lookup__all_missing.rs")]
#[case::dynamic_variant_missing("tests/ui/lookup__dynamic_variant_missing.rs")]
#[case::input_modality_missing("tests/ui/lookup__input_modality_missing.rs")]
fn ui_lookup_errors(#[case] path: &str) {
    trybuild::TestCases::new().compile_fail(path);
}
