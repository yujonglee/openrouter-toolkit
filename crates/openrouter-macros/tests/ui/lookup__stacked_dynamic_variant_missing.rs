use openrouter_macros::model_supports;

const MODEL: &str = model_supports!("perceptron/perceptron-mk1:exacto:nitro", param::tools);

fn main() {
    let _ = MODEL;
}
