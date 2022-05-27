use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::{quote, ToTokens};
use scraper::Selector;
use std::fmt::format;
use syn::{parse_macro_input, PathArguments, PathSegment, Type, TypePath};

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
        ty: syn::Type,
        ident: Option<syn::Ident>,
        select: Option<String>,
        extract: Option<H2sExtractionMethod>,
    }
    #[derive(Debug, FromMeta)]
    #[darling(rename_all = "snake_case")]
    pub enum H2sExtractionMethod {
        Attr(String),
        Text,
    }
    impl H2sExtractionMethod {
        fn desc(&self) -> String {
            match self {
                H2sExtractionMethod::Attr(a) => format!("attr=\"{a}\""),
                H2sExtractionMethod::Text => format!("text"),
            }
        }
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
                .expect(
                    format!(
                        "{} should be struct because it is deriving H2s",
                        ident.to_string()
                    )
                    .as_str(),
                )
                .fields
                .into_iter()
                .map(
                    |H2sFieldReceiver {
                         ty,
                         ident,
                         select,
                         extract,
                     }| {
                        // all fields must be named
                        let ident:&syn::Ident = ident
                            .as_ref()
                            .expect("all struct fields for h2s must be named.");
                        let field_name = ident.to_string();

                        let source = match &select {
                            Some(selector) => {
                                // check selector validity at compile time
                                Selector::parse(selector)
                                    .expect(&format!("invalid css selector: `{}`", selector));
                                // TODO cache parsed selector
                                quote!(
                                    source.select(
                                       &N::Selector::parse(#selector).map_err(|e|::h2s::ExtractionError::Unexpected(e))?
                                    )
                                )
                            }
                            None => quote!(source.clone()),
                        };
                        let args = match extract {
                            Some(H2sExtractionMethod::Attr(attr)) => {
                                quote!(::h2s::StringExtractionMethod::Attribute(#attr .to_string()))
                            }
                            Some(H2sExtractionMethod::Text) => {
                                quote!(::h2s::StringExtractionMethod::Text)
                            }
                            None => quote!(()),
                        };

                        let selector = select.as_ref().map(|a|quote!(Some(#a .to_string()))).unwrap_or_else(||quote!(None));
                        quote!(
                            #ident: ::h2s::macro_utils::adjust_and_parse::<_,N,_>(#source, &#args)
                                        .map_err(|e| ::h2s::ExtractionError::Child{
                                            context: ::h2s::Position::Struct{selector: #selector, field_name: #field_name .to_string()},
                                            error: Box::new(e),
                                        })
                        ?)
                    },
                );

            tokens.extend(quote! {
                impl ::h2s::FromHtml for #ident {
                    type Source<N: ::h2s::HtmlElementRef> = N;
                    type Args = ();
                    fn from_html<N: ::h2s::HtmlElementRef>(
                        source: &Self::Source<N>,
                        args: &Self::Args,
                    ) -> Result<Self, ::h2s::ExtractionError> {
                        use ::h2s::Selector;
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
