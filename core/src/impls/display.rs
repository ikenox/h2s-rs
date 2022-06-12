use crate::*;
use std::fmt::{Display, Formatter};

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root { message, cause } => match cause {
                Some(e) => write!(f, "{message}: {e}"),
                None => write!(f, "{message}"),
            },
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
                    write!(f, "({field_name})[{s}]")
                } else {
                    write!(f, "{field_name}")
                }
            }
        }
    }
}

impl Display for StructureUnmatched {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for TextExtractionFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position() {
        let case = vec![
            (Position::Index(3), "[3]"),
            (
                Position::Struct {
                    selector: Some(".a > .b".to_string()),
                    field_name: "bar".to_string(),
                },
                r#"(bar)[.a > .b]"#,
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
            (
                ParseError::Root {
                    message: "foo".to_string(),
                    cause: None,
                },
                "foo".to_string(),
            ),
            (
                ParseError::Root {
                    message: "foo".to_string(),
                    cause: Some("bar".to_string()),
                },
                "foo: bar".to_string(),
            ),
            // child
            {
                let p = Position::Index(3);
                let e = ParseError::Root {
                    message: "foo".to_string(),
                    cause: Some("bar".to_string()),
                };
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
