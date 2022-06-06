//! A set of internal utility methods that will be used in the auto-generated code on `FromHtml` derive macro.
//! You wouldn't call these methods directly in your code.

use crate::{
    ExtractAttribute, ExtractionError, FromHtml, HtmlElementRef, Position, Selector,
    StructureAdjuster,
};

pub fn extract_attribute(attr: &str) -> ExtractAttribute {
    ExtractAttribute(attr.to_string())
}

pub fn select<N: HtmlElementRef>(
    source: &N,
    selector: &'static str,
) -> Result<Vec<N>, ExtractionError> {
    // TODO cache parsed selector
    let selector = N::Selector::parse(selector).map_err(ExtractionError::Unexpected)?;
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
) -> Result<H, ExtractionError> {
    source
        .try_adjust()
        .map_err(ExtractionError::StructureUnmatched)
        .and_then(|s| H::from_html(&s, args))
        .map_err(|e| ExtractionError::Child {
            context: Position::Struct {
                selector: selector.map(|a| a.to_string()),
                field_name: field_name.to_string(),
            },
            error: Box::new(e),
        })
}
