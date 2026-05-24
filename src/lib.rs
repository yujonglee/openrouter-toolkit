use proc_macro::TokenStream;

mod expand;
mod input;
mod models;

macro_rules! define_model_param_macro {
    ($macro_name:ident, $required_parameter:literal) => {
        #[proc_macro]
        pub fn $macro_name(input: TokenStream) -> TokenStream {
            expand::model_parameter(input, $required_parameter)
        }
    };
}

define_model_param_macro!(structured_model, "structured_outputs");
define_model_param_macro!(response_format_model, "response_format");

#[proc_macro]
pub fn model_supports(input: TokenStream) -> TokenStream {
    expand::model_supports(input)
}
