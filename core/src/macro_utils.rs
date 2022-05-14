use std::fmt::{Display, Formatter};

use crate::{ExtractionError, FromHtml, HtmlNodeRef, Position, StructureAdjuster};

pub fn adjust_and_parse<H: FromHtml, N: HtmlNodeRef, S: StructureAdjuster<H::Source<N>>>(
    source: S,
    args: &H::Args,
) -> Result<H, ExtractionError> {
    H::from_html(
        &source
            .try_adjust()
            .map_err(ExtractionError::StructureUnmatched)?,
        args,
    )
    .map_err(|e| ExtractionError::Child {
        context: Position::None, // todo
        error: std::boxed::Box::new(e),
    })
}
