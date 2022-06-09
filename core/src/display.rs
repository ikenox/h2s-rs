use crate::*;
use std::fmt::{Display, Formatter};

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(detail) => {
                write!(f, "{detail}")
            }
            Self::Child { position, error } => {
                write!(f, "{position} -> {error}")
            }
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
            } => {
                if let Some(s) = selector {
                    write!(f, "[{s}]({field_name})")
                } else {
                    write!(f, "{field_name}")
                }
            }
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
                r#"[.a > .b](bar)"#,
            ),
            (
                Position::Struct {
                    selector: None,
                    field_name: "bar".to_string(),
                },
                "bar",
            ),
        ];

        for (p, msg) in case {
            assert_eq!(format!("{}", p).as_str(), msg);
        }
    }

    #[test]
    fn parse_error() {
        let case = vec![
            // root
            (ParseError::Root("foo".to_string()), "foo".to_string()),
            // child
            {
                let p = Position::Index(3);
                let e = ParseError::Root("foo".to_string());
                (
                    ParseError::Child {
                        position: p.clone(),
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
}
