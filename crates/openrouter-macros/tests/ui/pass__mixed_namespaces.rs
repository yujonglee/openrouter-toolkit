use openrouter_macros::model_supports;

const MODEL: &str = model_supports!(
    "x-ai/grok-build-0.1",
    param::tools,
    input::image,
    output::text,
);

fn main() {
    assert_eq!(MODEL, "x-ai/grok-build-0.1");
}
