use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("x-ai/grok-build-0.1", input::image);

fn main() {
    assert_eq!(MODEL, "x-ai/grok-build-0.1");
}
