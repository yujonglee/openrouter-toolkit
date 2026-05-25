use openrouter_check::{
    check, format_capability, parse_namespace, validate_capability_name, DiagnosticKind, Namespace,
};

#[test]
fn formats_unknown_model() {
    let diagnostics = check("example/not-a-real-model", &["param::tools"]).unwrap_err();

    assert_eq!(diagnostics[0].kind, DiagnosticKind::UnknownModel);
    assert_eq!(
        diagnostics[0].message,
        "unknown OpenRouter model `example/not-a-real-model`"
    );
}

#[test]
fn formats_unknown_namespace() {
    let error = parse_namespace("capability").unwrap_err();

    assert_eq!(
        error.to_string(),
        "unknown OpenRouter capability namespace `capability`; expected one of: param, input, output"
    );
}

#[test]
fn formats_unknown_capability_with_suggestion() {
    let error = validate_capability_name(Namespace::Param, "toolz").unwrap_err();

    assert_eq!(
        error.to_string(),
        "unknown OpenRouter capability `param::toolz`; did you mean `param::tools`?"
    );
}

#[test]
fn formats_unknown_capability_with_known_names() {
    let error = validate_capability_name(Namespace::Param, "zzzzzzzzzz").unwrap_err();

    assert_eq!(
        error.to_string(),
        "unknown OpenRouter capability `param::zzzzzzzzzz`; known param capabilities: frequency_penalty, include_reasoning, logit_bias, logprobs, max_completion_tokens, max_tokens, min_p, parallel_tool_calls, presence_penalty, reasoning, reasoning_effort, repetition_penalty, response_format, seed, stop, structured_outputs, temperature, tool_choice, tools, top_a, top_k, top_logprobs, top_p, verbosity, web_search_options"
    );
}

#[test]
fn formats_duplicate_capability() {
    let diagnostics = check("qwen/qwen3.7-max", &["param::tools", "param::tools"]).unwrap_err();

    assert_eq!(diagnostics[0].kind, DiagnosticKind::DuplicateCapability);
    assert_eq!(
        diagnostics[0].message,
        format!(
            "duplicate OpenRouter capability `{}`",
            format_capability(Namespace::Param, "tools")
        )
    );
}

#[test]
fn formats_single_missing_capability() {
    let diagnostics = check("perceptron/perceptron-mk1", &["param::tools"]).unwrap_err();

    assert_eq!(diagnostics[0].kind, DiagnosticKind::MissingCapability);
    assert_eq!(
        diagnostics[0].message,
        "OpenRouter model `perceptron/perceptron-mk1` does not support required capability(s): param::tools"
    );
}

#[test]
fn formats_multiple_missing_capabilities() {
    let diagnostics = check(
        "openrouter/pareto-code",
        &["param::structured_outputs", "param::tools"],
    )
    .unwrap_err();

    assert_eq!(diagnostics[0].kind, DiagnosticKind::MissingCapability);
    assert_eq!(
        diagnostics[0].message,
        "OpenRouter model `openrouter/pareto-code` does not support required capability(s): param::structured_outputs, param::tools"
    );
}
