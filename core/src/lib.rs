#![feature(generic_associated_types)]
mod impls;
pub mod macro_utils;
pub mod utils;

use std::fmt::{Debug, Display, Formatter};

pub trait FromHtml<'a, A: 'a>: Sized {
    type Source<N: HtmlElementRef>;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: A,
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
pub struct ExtractAttribute(pub String);

impl<'a> FromHtml<'a, ()> for String {
    type Source<N: HtmlElementRef> = N;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: (),
    ) -> Result<Self, ExtractionError> {
        Ok(source.text_contents())
    }
}

impl<'a> FromHtml<'a, &'a ExtractAttribute> for String {
    type Source<N: HtmlElementRef> = N;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &'a ExtractAttribute,
    ) -> Result<Self, ExtractionError> {
        source
            .get_attribute(&args.0)
            .map(|s| s.to_string())
            .ok_or_else(|| ExtractionError::AttributeNotFound(args.0.clone()))
    }
}
impl<'a, B: Copy + 'a, T: FromHtml<'a, B>, const A: usize> FromHtml<'a, B> for [T; A] {
    type Source<N: HtmlElementRef> = [T::Source<N>; A];

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: B,
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

impl<'a, A: Copy + 'a, T: FromHtml<'a, A>> FromHtml<'a, A> for Vec<T> {
    type Source<N: HtmlElementRef> = Vec<T::Source<N>>;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: A,
    ) -> Result<Self, ExtractionError> {
        source
            .into_iter()
            .enumerate()
            .map(|(i, n)| {
                T::from_html(n, args.clone()).map_err(|e| ExtractionError::Child {
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

impl<'a, A: 'a, T: FromHtml<'a, A>> FromHtml<'a, A> for Option<T> {
    type Source<N: HtmlElementRef> = Option<T::Source<N>>;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: A,
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

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub struct StructureUnmatched(String);

impl Display for StructureUnmatched {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "structure is different from expected: {}", self.0)
    }
}
