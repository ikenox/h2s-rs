use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use scraper::Selector;
use syn::parse_macro_input;
use syn::spanned::Spanned;

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

#[proc_macro_derive(FromHtml, attributes(h2s))]
pub fn derive(input: TokenStream) -> TokenStream {
    match FromHtmlStructReceiver::from_derive_input(&parse_macro_input!(input)) {
        Ok(receiver) => quote!(#receiver).into(),
        Err(e) => TokenStream::from(e.write_errors()),
    }
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
                build_implementation_token(ident, fields.iter())
            }
            Data::Enum(_) => {
                syn::Error::new(ident.span(), "FromHtml doesn't support enum".to_string())
                    .to_compile_error()
            }
        };

        tokens.extend(token_stream);
    }
}

fn build_implementation_token<'a>(
    ident: &syn::Ident,
    fields: impl Iterator<Item = &'a FieldInfo>,
) -> proc_macro2::TokenStream {
    let field_and_values = fields
        .map(|f| {
            let v = build_field_value_token(f);
            let ident = f.field_name.field();
            quote!(#ident: #v)
        })
        .collect::<Vec<_>>();
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

fn build_field_value_token(f: &FieldInfo) -> proc_macro2::TokenStream {
    let source = match f.source() {
        Ok(Source::Root) => quote!(source.clone()),
        Ok(Source::Select(selector)) => {
            quote!(::h2s::macro_utils::select::<N>(source,#selector)?)
        }
        Err(e) => return e.to_compile_error(),
    };

    let args = f.args();

    let selector = f
        .select
        .as_ref()
        .map(|a| quote!(::std::option::Option::Some(#a)))
        .unwrap_or_else(|| quote!(::std::option::Option::None));

    let field_name = f.field_name.as_string();

    quote!(::h2s::macro_utils::adjust_and_parse::<N,_,_,_>(#source, #args, #selector, #field_name)?)
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

    fn span_with_type(&self, ty: &syn::Type) -> proc_macro2::Span {
        match &self {
            FieldName::Named(ident) => ident.span().join(ty.span()),
            FieldName::Index(_) => None,
        }
        .unwrap_or_else(|| ty.span())
    }

    fn field(&self) -> proc_macro2::TokenStream {
        match &self {
            FieldName::Named(id) => quote!(#id),
            FieldName::Index(i) => {
                let i = syn::Index::from(*i);
                quote!(#i)
            }
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
    fn source(&self) -> Result<Source, syn::Error> {
        match &self.select {
            Some(selector) => {
                // check selector validity at compile time
                Selector::parse(selector).map_err(|_| {
                    syn::Error::new(
                        // TODO highlight the span of macro attribute, not field ident and type
                        self.field_name.span_with_type(&self.ty),
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
