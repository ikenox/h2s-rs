use crate::*;
use std::fmt::Display;

impl Display for ExtractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo
        match self {
            Self::StructureUnmatched(e) => {
                write!(f, "structure unmatched: {e}")
            }
            Self::AttributeNotFound(attr) => {
                write!(f, "attribute `{attr}` is not found")
            }
            Self::Child { context, error } => {
                write!(f, "{context} -> {error}")
            }
            Self::Unexpected(detail) => write!(f, "unexpected error: {}", detail),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Position::Index(i) => write!(f, "[{i}]"),
            Position::Struct {
                selector,
                field_name,
            } => write!(
                f,
                "({field_name}) {}",
                if let Some(s) = selector {
                    format!("@ `{s}`")
                } else {
                    "".to_string()
                }
            ),
        }
    }
}

impl Display for StructureUnmatched {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "structure is different from expected: {}", self.0)
    }
}
