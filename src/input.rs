use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{LitStr, Token};

pub(crate) struct ModelSupportsInput {
    pub(crate) model: LitStr,
    pub(crate) required_parameters: Punctuated<LitStr, Token![,]>,
}

impl Parse for ModelSupportsInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let model = input.parse()?;
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            return Err(input.error("expected at least one required OpenRouter parameter"));
        }

        let required_parameters = Punctuated::parse_separated_nonempty(input)?;

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        Ok(Self {
            model,
            required_parameters,
        })
    }
}
