use openrouter_toolkit::structured_model;

const MODEL_ID: &str = "qwen/qwen3.7-max";
const MODEL: &str = structured_model!(MODEL_ID);

fn main() {
    let _ = MODEL;
}
