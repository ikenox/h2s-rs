use std::fmt::{Display, Formatter};

use crate::{ExtractionError, FromHtml, HtmlElementRef, Position, StructureAdjuster};

pub fn adjust_and_parse<
    'a,
    N: HtmlElementRef,
    A: 'a,
    H: FromHtml<'a, A>,
    S: StructureAdjuster<H::Source<N>>,
>(
    source: S,
    args: A,
) -> Result<H, ExtractionError> {
    source
        .try_adjust()
        .map_err(ExtractionError::StructureUnmatched)
        .and_then(|s| H::from_html(&s, args))
}
