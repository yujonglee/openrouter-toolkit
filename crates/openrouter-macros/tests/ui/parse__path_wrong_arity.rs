use openrouter_macros::model_supports;

const MODEL: &str = model_supports!("qwen/qwen3.7-max", param::capability::tools);

fn main() {
    let _ = MODEL;
}
