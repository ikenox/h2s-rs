use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use scraper::Selector;
use syn::parse_macro_input;

#[proc_macro_derive(FromHtml, attributes(h2s))]
pub fn derive(input: TokenStream) -> TokenStream {
    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(h2s), supports(struct_any))]
    pub struct FromHtmlStructReceiver {
        ident: syn::Ident,
        data: darling::ast::Data<(), H2sFieldReceiver>,
    }

    #[derive(Debug, FromField)]
    #[darling(attributes(h2s))]
    pub struct H2sFieldReceiver {
        ident: Option<syn::Ident>,
        select: Option<String>,
        attr: Option<String>,
    }

    impl ToTokens for FromHtmlStructReceiver {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let Self {
                ref ident,
                ref data,
            } = *self;

            let field_and_values = data
                .as_ref()
                .take_struct()
                .unwrap_or_else(|| panic!(
                    "{} should be struct because it is deriving H2s",
                    ident,
                ))
                .fields
                .into_iter()
                .map(
                    |H2sFieldReceiver {
                         ident,
                         select,
                         attr,
                     }| {
                        // all fields must be named
                        let ident:&syn::Ident = ident
                            .as_ref()
                            .expect("All fields of the struct deriving FromHtml must have a field name");
                        let field_name = ident.to_string();

                        let source = match &select {
                            Some(selector) => {
                                // check selector validity at compile time
                                Selector::parse(selector).unwrap_or_else(|_| panic!("invalid css selector: `{}`", selector));
                                quote!(::h2s::macro_utils::select::<N>(source,#selector)?)
                            }
                            None => quote!(source.clone()),
                        };
                        let args = match attr {
                            Some(attr) => {
                                quote!(& ::h2s::macro_utils::extract_attribute(#attr))
                            }
                            None => quote!(()),
                        };
                        let selector = select.as_ref()
                            .map(|a|quote!(Some(#a)))
                            .unwrap_or_else(||quote!(None));
                        quote!(
                            #ident: ::h2s::macro_utils::adjust_and_parse::<N,_,_,_>(#source, #args, #selector, #field_name)?
                        )
                    },
                );

            tokens.extend(quote! {
                impl <'a> ::h2s::FromHtml<'a, ()> for #ident {
                    type Source<N: ::h2s::HtmlElementRef> = N;
                    fn from_html<N: ::h2s::HtmlElementRef>(
                        source: &Self::Source<N>,
                        args: (),
                    ) -> Result<Self, ::h2s::ExtractionError> {
                        Ok(Self{
                            #(#field_and_values),*
                        })
                    }
                }
            });
        }
    }

    let struct_receiver: FromHtmlStructReceiver =
        match FromHtmlStructReceiver::from_derive_input(&parse_macro_input!(input)) {
            Ok(a) => a,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };
    quote!(#struct_receiver).into()
}
