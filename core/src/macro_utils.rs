use std::fmt::{Display, Formatter};

use crate::{ExtractionError, FromHtml, HtmlNodeRef, Position, StructureAdjuster};

pub fn adjust_and_parse<H: FromHtml, N: HtmlNodeRef, S: StructureAdjuster<H::Source<N>>>(
    source: S,
    args: &H::Args,
) -> Result<H, ExtractionError> {
    source
        .try_adjust()
        .map_err(ExtractionError::StructureUnmatched)
        .and_then(|s| H::from_html(&s, args))
}
