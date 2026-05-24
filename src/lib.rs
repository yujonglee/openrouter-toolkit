use proc_macro::TokenStream;

mod expand;
mod input;
mod models;
#[cfg(test)]
mod test_support;

#[proc_macro]
pub fn model_supports(input: TokenStream) -> TokenStream {
    expand::model_supports(input)
}
