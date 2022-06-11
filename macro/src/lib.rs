use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use scraper::Selector;
use syn::parse_macro_input;
use syn::spanned::Spanned;

#[proc_macro_derive(FromHtml, attributes(h2s))]
pub fn derive(input: TokenStream) -> TokenStream {
    let struct_receiver: FromHtmlStructReceiver =
        match FromHtmlStructReceiver::from_derive_input(&parse_macro_input!(input)) {
            Ok(a) => a,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };
    quote!(#struct_receiver).into()
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
                let fields = fields
                    .into_iter()
                    .enumerate()
                    .map(|(i, r)| FieldInfo {
                        field_name: r
                            .ident
                            .clone()
                            .map(FieldName::Named)
                            .unwrap_or_else(|| FieldName::Index(i)),
                        select: r.select.clone(),
                        ty: r.ty.clone(),
                        attr: r.attr.clone(),
                    })
                    .collect::<Vec<_>>();
                impl_token_stream(ident, fields.iter())
            }
            Data::Enum(_) => {
                syn::Error::new(ident.span(), "FromHtml doesn't support enum".to_string())
                    .to_compile_error()
            }
        };

        tokens.extend(token_stream);
    }
}

fn impl_token_stream<'a>(
    ident: &syn::Ident,
    fields: impl Iterator<Item = &'a FieldInfo>,
) -> proc_macro2::TokenStream {
    let field_and_values = fields.map(field_name_and_value).collect::<Vec<_>>();
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

fn field_name_and_value(f: &FieldInfo) -> proc_macro2::TokenStream {
    let source = match f.source() {
        Ok(Source::Root) => quote!(source.clone()),
        Ok(Source::Select(selector)) => {
            quote!(::h2s::macro_utils::select::<N>(source,#selector)?)
        }
        Err(e) => return f.field_name_and_value(e.to_compile_error()),
    };

    let args = f.args();

    let selector = f
        .select
        .as_ref()
        .map(|a| quote!(::std::option::Option::Some(#a)))
        .unwrap_or_else(|| quote!(::std::option::Option::None));

    let field_name = f.field_name.as_string();

    f.field_name_and_value(
        quote!(::h2s::macro_utils::adjust_and_parse::<N,_,_,_>(#source, #args, #selector, #field_name)?),
    )
}

enum FieldName {
    Named(syn::Ident),
    Index(usize),
}

impl FieldName {
    fn as_string(&self) -> String {
        match &self {
            FieldName::Named(ident) => ident.to_string(),
            FieldName::Index(i) => i.to_string(),
        }
    }
}

struct FieldInfo {
    field_name: FieldName,
    ty: syn::Type,

    select: Option<String>,
    attr: Option<String>,
}

impl FieldInfo {
    fn field_name_and_value(&self, token: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match &self.field_name {
            FieldName::Named(id) => quote!(#id: #token),
            FieldName::Index(i) => {
                let i = syn::Index::from(*i);
                quote!(#i: #token)
            }
        }
    }

    fn source(&self) -> Result<Source, syn::Error> {
        match &self.select {
            Some(selector) => {
                // check selector validity at compile time
                Selector::parse(selector).map_err(|_| {
                    syn::Error::new(
                        // TODO highlight the span of macro attribute, not field ident and type
                        match &self.field_name {
                            FieldName::Named(ident) => ident.span().join(self.ty.span()),
                            FieldName::Index(_) => None,
                        }
                        .unwrap_or_else(|| self.ty.span()),
                        format!("invalid css selector: `{}`", selector),
                    )
                })?;
                Ok(Source::Select(selector.to_string()))
            }
            None => Ok(Source::Root),
        }
    }

    fn args(&self) -> proc_macro2::TokenStream {
        match &self.attr {
            Some(attr) => {
                quote!(& ::h2s::macro_utils::extract_attribute(#attr))
            }
            None => quote!(()),
        }
    }
}

enum Source {
    Select(String),
    Root,
}
