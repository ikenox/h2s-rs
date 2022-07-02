use crate::adjuster::AdjustStructureError;
use crate::text_extractor::TextExtractionError;
use crate::ParseSelectorError;
use std::fmt::{Display, Formatter};

/// Similar with std::convert::Infallible
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Never {}

impl ParseSelectorError for Never {}

impl AdjustStructureError for Never {}

impl TextExtractionError for Never {}

impl Display for Never {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // never reached here
        write!(f, "")
    }
}
