use darling::{FromDeriveInput, FromField};
use h2s_core::Extractor;
use kuchiki::Selectors;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_derive(H2s, attributes(h2s))]
pub fn derive(input: TokenStream) -> TokenStream {
    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(h2s), supports(struct_any))]
    pub struct H2sStructReceiver {
        ident: syn::Ident,
        data: darling::ast::Data<(), H2sFieldReceiver>,
    }
    #[derive(Debug, FromField)]
    #[darling(attributes(h2s))]
    pub struct H2sFieldReceiver {
        ident: Option<syn::Ident>,
        select: String,
    }

    impl ToTokens for H2sStructReceiver {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let Self {
                ref ident,
                ref data,
            } = *self;

            let field_and_values = data
                .as_ref()
                .take_struct()
                .expect(
                    format!(
                        "{} should be struct because it is deriving H2s",
                        ident.to_string()
                    )
                    .as_str(),
                )
                .fields
                .into_iter()
                .enumerate()
                .map(|(i, H2sFieldReceiver { ident, select })| {
                    // all fields must be named
                    let ident = ident
                        .as_ref()
                        .expect(&format!("all struct fields for h2s must be named."));
                    // check selector validity at compile time
                    Selectors::compile(&select)
                        .expect(&format!("invalid css selector: `{}`", select));

                    let selector = quote!(#select.to_string());
                    quote!(#ident: ::h2s_core::Extractor{selector: #selector}.extract(node)?)
                })
                .collect::<Vec<_>>();

            tokens.extend(quote! {
                impl ::h2s::H2s for #ident {
                    fn parse(node: &::kuchiki::NodeRef) -> Result<Self, ::h2s::H2sError> {
                        Ok(Self{
                            #(#field_and_values),*
                        })
                    }
                }
            });
        }
    }

    let struct_receiver: H2sStructReceiver =
        H2sStructReceiver::from_derive_input(&parse_macro_input!(input)).unwrap();
    quote!(#struct_receiver).into()
}
