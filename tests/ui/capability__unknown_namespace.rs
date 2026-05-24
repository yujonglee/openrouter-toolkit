use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("qwen/qwen3.7-max", capability::tools);

fn main() {
    let _ = MODEL;
}
