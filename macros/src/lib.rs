use quote::ToTokens;
mod parser;

use parser::RawNode;

#[proc_macro]
pub fn rsx(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse2::<RawNode>(tokens.into())
        .unwrap()
        .into_token_stream()
        .into()
}
