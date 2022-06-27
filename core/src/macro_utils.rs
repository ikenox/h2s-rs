//! A set of internal utility methods that will be used in the auto-generated code on `FromHtml` derive macro.
//! These methods are shorthands to reduce codes in the `quote!` macro. It improves development experience with IDE.
//! You wouldn't call these methods directly in your code.

use crate::impls::adjuster::StructureAdjuster;
use crate::impls::from_html::{StructErrorCause, StructFieldError};
use crate::impls::text_extractor::ExtractAttribute;
use crate::FromHtmlError;
use crate::{FromHtml, HtmlNode, Selector};

pub fn extract_attribute(attr: &str) -> ExtractAttribute {
    ExtractAttribute {
        name: attr.to_string(),
    }
}

pub fn select<N: HtmlNode>(source: &N, selector: &'static str) -> Vec<N> {
    // TODO cache parsed selector
    let selector = N::Selector::parse(selector)
        // this should be never failed because the selector validity has been checked at compile-time
        // TODO avoid unwrap
        .unwrap();
    source.select(&selector)
}

impl FromHtmlError for Box<dyn FromHtmlError> {}

pub fn adjust_and_parse<N: HtmlNode, A, T: FromHtml<A>, S: StructureAdjuster<T::Source<N>>>(
    source: S,
    args: &A,
    selector: Option<&'static str>,
    field_name: &'static str,
) -> Result<T, Box<dyn FromHtmlError>> {
    source
        .try_adjust()
        .map_err(StructErrorCause::StructureUnmatched)
        .and_then(|s| T::from_html(&s, args).map_err(StructErrorCause::ParseError))
        .map_err(|e| StructFieldError {
            field_name: field_name.to_string(),
            selector: selector.map(|a| a.to_string()),
            error: e,
        })
        .map_err(|e| Box::new(e) as Box<dyn FromHtmlError>)
}
