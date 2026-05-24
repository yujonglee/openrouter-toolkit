use openrouter_toolkit::structured_model;

const MODEL: &str = structured_model!("qwen/qwen3.7-max");

fn main() {
    assert_eq!(MODEL, "qwen/qwen3.7-max");
}
