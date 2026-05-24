use openrouter_macros::model_supports;

const MODEL: &str = model_supports!("openai/gpt-5.4-image-2", output::image);

fn main() {
    assert_eq!(MODEL, "openai/gpt-5.4-image-2");
}
