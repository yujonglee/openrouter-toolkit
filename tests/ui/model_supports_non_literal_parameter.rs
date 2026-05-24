use openrouter_toolkit::model_supports;

const PARAMETER: &str = "tools";
const MODEL: &str = model_supports!("qwen/qwen3.7-max", PARAMETER);

fn main() {
    let _ = MODEL;
}
