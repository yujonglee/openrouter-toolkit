use rstest::rstest;
use syn::punctuated::Punctuated;
use syn::{Path, Token};

use openrouter_check::{format_capabilities, format_capability};
use openrouter_models::Namespace;

use super::{parse_capabilities, parse_capability_path};

fn parse_path(path: &str) -> Path {
    syn::parse_str(path).expect("test path should parse")
}

fn parse_path_list(paths: &[&str]) -> Punctuated<Path, Token![,]> {
    let mut parsed = Punctuated::new();
    for path in paths {
        parsed.push(parse_path(path));
    }
    parsed
}

#[rstest]
#[case::param("param::tools", Namespace::Param, "tools")]
#[case::input("input::image", Namespace::Input, "image")]
#[case::output("output::text", Namespace::Output, "text")]
fn parses_capability_path(
    #[case] path: &str,
    #[case] expected_namespace: Namespace,
    #[case] expected_name: &str,
) {
    let parsed = parse_capability_path(&parse_path(path)).expect("path should parse");

    assert_eq!(parsed, (expected_namespace, expected_name.to_owned()));
}

#[rstest]
#[case::leading_colon("::param::tools", "expected OpenRouter capability path")]
#[case::single_segment("tools", "expected OpenRouter capability path")]
#[case::too_many_segments("param::cap::tools", "expected OpenRouter capability path")]
#[case::generic_arguments(
    "param::<T>::tools",
    "OpenRouter capability paths cannot contain generic arguments"
)]
#[case::unknown_namespace(
    "weird::tools",
    "unknown OpenRouter capability namespace `weird`; expected one of: param, input, output"
)]
#[case::unknown_name_suggestion(
    "param::toolz",
    "unknown OpenRouter capability `param::toolz`; did you mean `param::tools`?"
)]
fn rejects_invalid_capability_path(#[case] path: &str, #[case] expected_message: &str) {
    let error = parse_capability_path(&parse_path(path)).expect_err("path should be invalid");

    assert!(
        error.to_string().contains(expected_message),
        "expected `{}` to contain `{}`",
        error,
        expected_message,
    );
}

#[test]
fn rejects_duplicate_capabilities_in_same_namespace() {
    let paths = parse_path_list(&["param::tools", "param::tools"]);
    let error = parse_capabilities(&paths).expect_err("duplicates should fail");

    assert_eq!(
        error.to_string(),
        "duplicate OpenRouter capability `param::tools`"
    );
}

#[test]
fn accepts_same_name_in_different_namespaces() {
    let paths = parse_path_list(&["input::image", "output::image"]);
    let parsed = parse_capabilities(&paths).expect("cross-namespace names are distinct");
    let actual: Vec<_> = parsed
        .iter()
        .map(|capability| (capability.namespace, capability.name.as_str()))
        .collect();

    assert_eq!(
        actual,
        vec![(Namespace::Input, "image"), (Namespace::Output, "image")]
    );
}

#[test]
fn preserves_capability_order() {
    let paths = parse_path_list(&["output::text", "param::tools", "input::image"]);
    let parsed = parse_capabilities(&paths).expect("paths should parse");
    let actual: Vec<_> = parsed
        .iter()
        .map(|capability| (capability.namespace, capability.name.as_str()))
        .collect();

    assert_eq!(
        actual,
        vec![
            (Namespace::Output, "text"),
            (Namespace::Param, "tools"),
            (Namespace::Input, "image"),
        ],
    );
}

#[rstest]
#[case::param(Namespace::Param, "tools", "param::tools")]
#[case::input(Namespace::Input, "image", "input::image")]
#[case::output(Namespace::Output, "text", "output::text")]
fn formats_capability(#[case] namespace: Namespace, #[case] name: &str, #[case] expected: &str) {
    assert_eq!(format_capability(namespace, name), expected);
}

#[test]
fn formats_multiple_capabilities() {
    assert_eq!(
        format_capabilities([(Namespace::Param, "tools"), (Namespace::Input, "image")]),
        "param::tools, input::image",
    );
}
