use openrouter_toolkit::response_format_model;

const MODEL_ID: &str = "inclusionai/ring-2.6-1t";
const MODEL: &str = response_format_model!(MODEL_ID);

fn main() {
    let _ = MODEL;
}
