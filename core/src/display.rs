use crate::*;
use std::fmt::{Display, Formatter};

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StructureUnmatched(e) => {
                write!(f, "{e}")
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
                "[{field_name}]{}",
                if let Some(s) = selector {
                    format!("(\"{s}\")")
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

#[cfg(test)]
mod test {
    use crate::{ParseError, Position, StructureUnmatched};

    #[test]
    fn position() {
        let case = vec![
            (Position::Index(3), "[3]"),
            (
                Position::Struct {
                    selector: Some(".a > .b".to_string()),
                    field_name: "bar".to_string(),
                },
                r#"[bar](".a > .b")"#,
            ),
            (
                Position::Struct {
                    selector: None,
                    field_name: "bar".to_string(),
                },
                "[bar]",
            ),
        ];

        for (p, msg) in case {
            assert_eq!(format!("{}", p).as_str(), msg);
        }
    }

    #[test]
    fn parse_error() {
        let case = vec![
            {
                let e = StructureUnmatched("foo".to_string());
                (ParseError::StructureUnmatched(e.clone()), format!("{e}"))
            },
            (
                ParseError::AttributeNotFound("foo".to_string()),
                "attribute `foo` is not found".to_string(),
            ),
            (
                ParseError::Unexpected("foo".to_string()),
                "unexpected error: foo".to_string(),
            ),
            {
                let p = Position::Index(3);
                let e = ParseError::Unexpected("foo".to_string());
                (
                    ParseError::Child {
                        context: p.clone(),
                        error: Box::new(e.clone()),
                    },
                    format!("{p} -> {e}"),
                )
            },
        ];

        for (e, msg) in case {
            assert_eq!(format!("{}", e), msg);
        }
    }

    #[test]
    fn structure_unmatched() {}
}
