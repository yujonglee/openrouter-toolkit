use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("example/not-a-real-model", "tools");

fn main() {
    let _ = MODEL;
}
