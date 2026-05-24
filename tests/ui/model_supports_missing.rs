use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("perceptron/perceptron-mk1", "structured_outputs", "tools");

fn main() {
    let _ = MODEL;
}
