//! A set of internal utility methods that will be used in the auto-generated code on `FromHtml` derive macro.
//! You wouldn't call these methods directly in your code.

use crate::from_html::ExtractAttribute;
use crate::{FromHtml, HtmlElementRef, ParseError, Position, Selector, StructureAdjuster};

pub fn extract_attribute(attr: &str) -> ExtractAttribute {
    ExtractAttribute(attr.to_string())
}

pub fn select<N: HtmlElementRef>(source: &N, selector: &'static str) -> Result<Vec<N>, ParseError> {
    // TODO cache parsed selector
    let selector = N::Selector::parse(selector).map_err(|_| {
        ParseError::Root(format!(
            "unexpected error occurs while parsing CSS selector"
        ))
    })?;
    Ok(source.select(&selector))
}

pub fn adjust_and_parse<
    'a,
    N: HtmlElementRef,
    A: 'a,
    H: FromHtml<'a, A>,
    S: StructureAdjuster<H::Source<N>>,
>(
    source: S,
    args: A,
    selector: Option<&'static str>,
    field_name: &'static str,
) -> Result<H, ParseError> {
    source
        .try_adjust()
        .map_err(|e| ParseError::Root(format!("failed to adjust structure: {e}")))
        .and_then(|s| H::from_html(&s, args))
        .map_err(|e| ParseError::Child {
            position: Position::Struct {
                selector: selector.map(|a| a.to_string()),
                field_name: field_name.to_string(),
            },
            error: Box::new(e),
        })
}
