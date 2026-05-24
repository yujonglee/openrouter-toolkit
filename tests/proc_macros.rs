#[test]
fn validates_proc_macro_model_literals() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/structured_supported.rs");
    t.compile_fail("tests/ui/structured_unsupported.rs");
    t.compile_fail("tests/ui/structured_unknown.rs");
    t.compile_fail("tests/ui/structured_non_literal.rs");

    t.pass("tests/ui/response_format_supported.rs");
    t.pass("tests/ui/response_format_dynamic_variant.rs");
    t.compile_fail("tests/ui/response_format_unsupported.rs");
    t.compile_fail("tests/ui/response_format_unknown.rs");
    t.compile_fail("tests/ui/response_format_non_literal.rs");

    t.pass("tests/ui/model_supports_single.rs");
    t.pass("tests/ui/model_supports_multiple.rs");
    t.pass("tests/ui/model_supports_dynamic_variant.rs");
    t.compile_fail("tests/ui/model_supports_missing.rs");
    t.compile_fail("tests/ui/model_supports_dynamic_variant_missing.rs");
    t.compile_fail("tests/ui/model_supports_static_variant_unknown.rs");
    t.compile_fail("tests/ui/model_supports_all_missing.rs");
    t.compile_fail("tests/ui/model_supports_empty.rs");
    t.compile_fail("tests/ui/model_supports_no_parameters.rs");
    t.compile_fail("tests/ui/model_supports_unknown.rs");
    t.compile_fail("tests/ui/model_supports_non_literal_model.rs");
    t.compile_fail("tests/ui/model_supports_non_literal_parameter.rs");
}
