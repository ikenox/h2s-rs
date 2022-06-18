//! A set of internal utility methods that will be used in the auto-generated code on `FromHtml` derive macro.
//! These methods are shorthands to reduce codes in the `quote!` macro. It improves development experience with IDE.
//! You wouldn't call these methods directly in your code.

use crate::ExtractAttribute;
use crate::{FromHtml, HtmlNode, ParseError, Position, Selector, StructureAdjuster};

pub fn extract_attribute(attr: &str) -> ExtractAttribute {
    ExtractAttribute(attr.to_string())
}

pub fn select<N: HtmlNode>(source: &N, selector: &'static str) -> Result<Vec<N>, ParseError> {
    // TODO cache parsed selector
    let selector = N::Selector::parse(selector).map_err(|_| ParseError::Root {
        message: "unexpected error occurs while parsing CSS selector".to_string(),
        cause: None,
    })?;
    Ok(source.select(&selector))
}

pub fn adjust_and_parse<
    'a,
    N: HtmlNode,
    A: 'a,
    T: FromHtml<'a, A>,
    S: StructureAdjuster<T::Source<N>>,
>(
    source: S,
    args: A,
    selector: Option<&'static str>,
    field_name: &'static str,
) -> Result<T, ParseError> {
    source
        .try_adjust()
        .map_err(|e| ParseError::Root {
            message: "failed to adjust structure".to_string(),
            cause: Some(format!("{}", e)),
        })
        .and_then(|s| T::from_html(&s, args))
        .map_err(|e| ParseError::Child {
            position: Position::Struct {
                selector: selector.map(|a| a.to_string()),
                field_name: field_name.to_string(),
            },
            error: Box::new(e),
        })
}
