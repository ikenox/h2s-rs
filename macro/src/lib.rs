use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use scraper::Selector;
use syn::parse_macro_input;
use syn::spanned::Spanned;

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
    data: darling::ast::Data<(), H2sFieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(h2s))]
struct H2sFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    select: Option<String>,
    attr: Option<String>,
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
                quote! {
                    impl <'a> ::h2s::FromHtml<'a, ()> for #ident {
                        type Source<N: ::h2s::HtmlElementRef> = N;
                        fn from_html<N: ::h2s::HtmlElementRef>(
                            source: &Self::Source<N>,
                            args: (),
                        ) -> Result<Self, ::h2s::ParseError> {
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
        let source = match &self.select {
            Some(selector) => {
                // check selector validity at compile time
                if Selector::parse(selector).is_err() {
                    return syn::Error::new(
                        // TODO highlight the span of macro attribute, not field ident and type
                        self.ident
                            .as_ref()
                            .and_then(|id| id.span().join(self.ty.span()))
                            .unwrap_or_else(|| self.ty.span()),
                        format!("invalid css selector: `{}`", selector),
                    )
                    .to_compile_error();
                }
                quote!(::h2s::macro_utils::select::<N>(source,#selector)?)
            }
            None => quote!(source.clone()),
        };

        let args = match &self.attr {
            Some(attr) => {
                quote!(& ::h2s::macro_utils::extract_attribute(#attr))
            }
            None => quote!(()),
        };

        let selector = self
            .select
            .as_ref()
            .map(|a| quote!(::std::option::Option::Some(#a)))
            .unwrap_or_else(|| quote!(::std::option::Option::None));

        quote!(::h2s::macro_utils::adjust_and_parse::<N,_,_,_>(#source, #args, #selector, #field_name)?)
    }
}
