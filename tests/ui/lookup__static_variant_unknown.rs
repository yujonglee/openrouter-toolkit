use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("moonshotai/kimi-k2-0905:free", param::tools);

fn main() {
    let _ = MODEL;
}
