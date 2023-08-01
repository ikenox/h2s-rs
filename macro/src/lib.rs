//! A macro part of [h2s].
//! [h2s]: https://github.com/ikenox/h2s-rs

use proc_macro::TokenStream;

use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use quote::{quote, ToTokens};
use scraper::Selector;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr};

#[proc_macro_derive(FromHtml, attributes(h2s))]
pub fn derive(input: TokenStream) -> TokenStream {
    match FromHtmlStructReceiver::from_derive_input(&parse_macro_input!(input)) {
        Ok(struct_receiver) => quote!(#struct_receiver).into(),
        Err(e) => TokenStream::from(e.write_errors()),
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(h2s), supports(struct_any))]
struct FromHtmlStructReceiver {
    ident: syn::Ident,
    data: Data<(), H2sFieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(h2s))]
struct H2sFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    select: Option<String>,
    extractor: Option<Expr>,
    // TODO attr is a shorthand of specific extractor
    //      so it's better to represent that user cannot specify both
    attr: Option<String>,
    // text: bool,
}

impl ToTokens for FromHtmlStructReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            ref ident,
            ref data,
        } = *self;

        let token_stream = match data.as_ref() {
            Data::Struct(fields) => {
                let field_and_values = fields
                    .into_iter()
                    .enumerate()
                    .map(|(i, r)| r.build_field_and_value(i));
                // TODO Avoid using trait object
                quote! {
                    impl ::h2s::FromHtml for #ident {
                        type Error = ::h2s::FieldError;

                        fn from_html<E>(input: E) -> Result<Self, Self::Error>
                        where
                            E: ::h2s::html::HtmlElement
                        {
                            Ok(Self{
                                #(#field_and_values),*
                            })
                        }
                    }
                }
            }
            Data::Enum(_) => {
                syn::Error::new(ident.span(), "FromHtml doesn't support enum".to_string())
                    .to_compile_error()
            }
        };

        tokens.extend(token_stream);
    }
}

impl H2sFieldReceiver {
    fn build_field_and_value(&self, index: usize) -> proc_macro2::TokenStream {
        let (ident, field_name_str) = match &self.ident {
            Some(id) => (quote!(#id), id.to_string()),
            None => {
                let i = syn::Index::from(index);
                (quote!(#i), index.to_string())
            }
        };
        let value = self.build_value(&field_name_str);
        quote!(#ident: #value)
    }

    fn build_value(&self, field_name: &String) -> proc_macro2::TokenStream {
        let selector = match &self.select {
            Some(selector) => {
                // check selector validity at compile time
                if Selector::parse(selector).is_err() {
                    return syn::Error::new(
                        // TODO highlight the span of macro attribute, not field ident and type
                        self.ident
                            .as_ref()
                            .and_then(|id| id.span().join(self.ty.span()))
                            .unwrap_or_else(|| self.ty.span()),
                        format!("invalid css selector: `{selector}`"),
                    )
                    .to_compile_error();
                }
                quote!(::h2s::element_selector::Select{ selector: #selector.to_string() })
            }
            None => quote!(::h2s::element_selector::Root),
        };

        // TODO user‐unfriendly error message is shown when argument is mismatched
        let extraction_method = if let Some(attr) = self.attr.as_ref() {
            quote!(::h2s::macro_utils::extraction_method(::h2s::extraction_method::ExtractAttribute{ name: #attr .to_string() }))
        } else if let Some(a) = self.extractor.as_ref() {
            quote!(::h2s::macro_utils::extraction_method(#a))
        } else {
            quote!(::h2s::macro_utils::default_extraction_method::<E, _>())
        };

        quote!({
            let field_name = #field_name.to_string();
            let selector = #selector;
            let extraction_method = #extraction_method;
            ::h2s::macro_utils::process_field(&input, selector, extraction_method)
                .map_err(|error| ::h2s::FieldError {
                    field_name,
                    error: Box::new(error),
                })?
        })
    }
}
