use openrouter_toolkit::model_supports;

#[test]
fn reexports_model_supports_macro() {
    const MODEL: &str = model_supports!("qwen/qwen3.7-max", param::tools);

    assert_eq!(MODEL, "qwen/qwen3.7-max");
}

#[test]
fn rejects_unknown_model_at_compile_time() {
    trybuild::TestCases::new().compile_fail("tests/ui/lookup__unknown_model.rs");
}
