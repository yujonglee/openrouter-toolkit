use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{LitStr, Path, Token};

pub(crate) struct ModelSupportsInput {
    pub(crate) model: LitStr,
    pub(crate) required_capabilities: Punctuated<Path, Token![,]>,
}

impl Parse for ModelSupportsInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let model = input.parse()?;
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            return Err(input.error("expected at least one required OpenRouter capability"));
        }

        let required_capabilities = Punctuated::parse_terminated(input)?;
        if required_capabilities.is_empty() {
            return Err(input.error("expected at least one required OpenRouter capability"));
        }

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        Ok(Self {
            model,
            required_capabilities,
        })
    }
}

#[cfg(test)]
mod tests;
