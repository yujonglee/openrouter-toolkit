use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("qwen/qwen3.7-max", param::structured_outputs, param::tools);

fn main() {
    assert_eq!(MODEL, "qwen/qwen3.7-max");
}
