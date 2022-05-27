#![feature(generic_associated_types)]
mod impls;
pub mod macro_utils;
pub mod utils;

use std::fmt::{Debug, Display, Formatter};

pub trait FromHtml: Sized {
    type Source<N: HtmlElementRef>;
    type Args;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError>;
}

pub enum StringExtractionMethod {
    Text,
    Attribute(String),
}

pub trait StructureAdjuster<N> {
    fn try_adjust(self) -> Result<N, StructureUnmatched>;
}

impl<N> StructureAdjuster<N> for N {
    fn try_adjust(self) -> Result<N, StructureUnmatched> {
        Ok(self)
    }
}

impl<N> StructureAdjuster<N> for Vec<N> {
    fn try_adjust(mut self) -> Result<N, StructureUnmatched> {
        if self.len() > 1 {
            Err(StructureUnmatched(format!(
                "expected exactly one element, but found {} elements",
                self.len()
            )))
        } else {
            self.pop().ok_or_else(|| {
                StructureUnmatched(format!(
                    "expected exactly one element, but no element found",
                ))
            })
        }
    }
}

impl<N, const A: usize> StructureAdjuster<[N; A]> for Vec<N> {
    fn try_adjust(self) -> Result<[N; A], StructureUnmatched> {
        self.try_into().map_err(|v: Vec<_>| {
            StructureUnmatched(format!(
                "expected exactly {} elements, but found {} elements",
                A,
                v.len()
            ))
        })
    }
}

impl<N> StructureAdjuster<Option<N>> for Vec<N> {
    fn try_adjust(mut self) -> Result<Option<N>, StructureUnmatched> {
        if self.len() > 1 {
            Err(StructureUnmatched(format!(
                "expected at most one element, but found {} elements",
                self.len()
            )))
        } else {
            Ok(self.pop())
        }
    }
}

impl FromHtml for String {
    type Source<N: HtmlElementRef> = N;
    type Args = StringExtractionMethod;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        match args {
            StringExtractionMethod::Text => Ok(source.text_contents()),
            StringExtractionMethod::Attribute(attr) => source
                .get_attribute(attr)
                .map(|s| s.to_string())
                .ok_or_else(|| ExtractionError::AttributeNotFound(attr.clone())),
        }
    }
}
impl<T: FromHtml, const A: usize> FromHtml for [T; A] {
    type Source<N: HtmlElementRef> = [T::Source<N>; A];
    type Args = T::Args;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        let v = source
            .iter()
            .enumerate()
            .map(|(i, n)| {
                T::from_html(n, args).map_err(|e| ExtractionError::Child {
                    context: Position::Index(i),
                    error: Box::new(e),
                })
            })
            .fold(Ok(vec![]), |acc, res| {
                acc.and_then(|mut list| {
                    res.map(|val| {
                        list.push(val);
                        list
                    })
                })
            })?;

        // this conversion should never fail
        v.try_into().map_err(|_| {
            ExtractionError::Unexpected(format!("vec to array conversion unexpectedly failed"))
        })
    }
}

impl<T: FromHtml> FromHtml for Vec<T> {
    type Source<N: HtmlElementRef> = Vec<T::Source<N>>;
    type Args = T::Args;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        source
            .into_iter()
            .enumerate()
            .map(|(i, n)| {
                T::from_html(n, args).map_err(|e| ExtractionError::Child {
                    context: Position::Index(i),
                    error: Box::new(e),
                })
            })
            // unwrapping results
            .fold(Ok(vec![]), |acc, res| {
                acc.and_then(|mut list| {
                    res.map(|val| {
                        list.push(val);
                        list
                    })
                })
            })
    }
}

impl<T: FromHtml> FromHtml for Option<T> {
    type Source<N: HtmlElementRef> = Option<T::Source<N>>;
    type Args = T::Args;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        source
            .as_ref()
            .map(|n| T::from_html(n, args))
            .map_or(Ok(None), |v| v.map(Some))
    }
}
pub trait Selector: Sized {
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, String>;
}

// todo not force to clone?
pub trait HtmlElementRef: Sized + Clone {
    type Selector: Selector;
    fn select(&self, sel: &Self::Selector) -> Vec<Self>;
    fn text_contents(&self) -> String;
    fn get_attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str>;
}

#[derive(Debug)]
pub enum ExtractionError {
    Unexpected(String),
    StructureUnmatched(StructureUnmatched),
    AttributeNotFound(String),
    Child {
        context: Position,
        error: Box<ExtractionError>,
    },
}

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
                // todo
                // write!(f, "{source} $ {args} -> {error}")
                write!(f, "{context} -> {error}")
            }
            Self::Unexpected(detail) => write!(f, "unexpected error: {}", detail),
        }
    }
}

#[derive(Debug)]
pub enum Position {
    Index(usize),
    Struct {
        selector: Option<String>,
        field_name: String,
    },
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

#[derive(Debug)]
pub struct StructureUnmatched(String);

impl Display for StructureUnmatched {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "structure is different from expected: {}", self.0)
    }
}
