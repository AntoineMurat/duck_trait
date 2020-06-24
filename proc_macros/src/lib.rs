use quote::quote;
use proc_macro::TokenStream;
extern crate proc_macro;

#[proc_macro]
pub fn before_semicolon(tokens: TokenStream) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::from(tokens).into_iter();
    let callback = tokens.next().unwrap();
    let before = tokens.take_while(|token| &token.to_string() != &";")
        .collect::<proc_macro2::TokenStream>();
    let output = quote! {
        #callback!(#before);
    };
    TokenStream::from(output)
}

#[proc_macro]
pub fn after_semicolon(tokens: TokenStream) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::from(tokens).into_iter();
    let callback = tokens.next().unwrap();
    let after = tokens.skip_while(|token| &token.to_string() != &";")
        .skip(1)
        .collect::<proc_macro2::TokenStream>();
    let output = quote! {
        #callback!(#after);
    };
    TokenStream::from(output)
}