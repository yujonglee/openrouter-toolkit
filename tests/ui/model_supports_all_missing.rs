use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("openrouter/pareto-code", "structured_outputs", "tools");

fn main() {
    let _ = MODEL;
}
