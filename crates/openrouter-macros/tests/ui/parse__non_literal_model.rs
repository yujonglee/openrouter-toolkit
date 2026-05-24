use openrouter_macros::model_supports;

const MODEL_ID: &str = "qwen/qwen3.7-max";
const MODEL: &str = model_supports!(MODEL_ID, param::tools);

fn main() {
    let _ = MODEL;
}
