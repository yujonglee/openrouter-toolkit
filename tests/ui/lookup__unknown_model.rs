use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("example/not-a-real-model", param::tools);

fn main() {
    let _ = MODEL;
}
