use openrouter_toolkit::model_supports;

#[test]
fn reexports_model_supports_macro() {
    const MODEL: &str = model_supports!("qwen/qwen3.7-max", param::tools);

    assert_eq!(MODEL, "qwen/qwen3.7-max");
}
