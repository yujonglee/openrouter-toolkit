use rstest::rstest;

use super::ModelSupportsInput;

#[rstest]
#[case::single(r#""m", param::tools"#, "m", 1)]
#[case::trailing_comma(r#""m", param::tools,"#, "m", 1)]
#[case::multiple(r#""m", input::x, output::y"#, "m", 2)]
fn parses_model_supports_input(
    #[case] input: &str,
    #[case] expected_model: &str,
    #[case] expected_capability_count: usize,
) {
    let parsed = syn::parse_str::<ModelSupportsInput>(input).expect("input should parse");

    assert_eq!(parsed.model.value(), expected_model);
    assert_eq!(
        parsed.required_capabilities.len(),
        expected_capability_count
    );
}

#[rstest]
#[case::missing_comma(r#""m""#, "expected `,`")]
#[case::no_capabilities(r#""m","#, "expected at least one required OpenRouter capability")]
#[case::non_literal_model(r#"MODEL_ID, param::tools"#, "expected string literal")]
#[case::trailing_junk(r#""m", param::tools, ,"#, "expected identifier")]
fn rejects_invalid_model_supports_input(#[case] input: &str, #[case] expected_message: &str) {
    let error = match syn::parse_str::<ModelSupportsInput>(input) {
        Ok(_) => panic!("input should fail"),
        Err(error) => error,
    };

    assert!(
        error.to_string().contains(expected_message),
        "expected `{}` to contain `{}`",
        error,
        expected_message,
    );
}
