use openrouter_toolkit::response_format_model;

const MODEL: &str = response_format_model!("inclusionai/ring-2.6-1t");

fn main() {
    assert_eq!(MODEL, "inclusionai/ring-2.6-1t");
}
