use openrouter_macros::model_supports;

const MODEL: &str = model_supports!("openrouter/pareto-code", param::structured_outputs, param::tools);

fn main() {
    let _ = MODEL;
}
