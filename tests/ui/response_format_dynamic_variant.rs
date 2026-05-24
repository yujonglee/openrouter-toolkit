use openrouter_toolkit::response_format_model;

const MODEL: &str = response_format_model!("openai/gpt-5.5:nitro");

fn main() {
    assert_eq!(MODEL, "openai/gpt-5.5:nitro");
}
