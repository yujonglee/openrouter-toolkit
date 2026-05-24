use openrouter_toolkit::model_supports;

const MODEL: &str = model_supports!("moonshotai/kimi-k2-0905:exacto", param::tools);

fn main() {
    assert_eq!(MODEL, "moonshotai/kimi-k2-0905:exacto");
}
