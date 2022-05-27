#![feature(generic_associated_types)]
mod impls;
pub mod macro_utils;
pub mod utils;

use std::fmt::{Debug, Display, Formatter};

pub trait FromHtml: Sized {
    type Source<N: HtmlNodeRef>;
    type Args;

    fn from_html<N: HtmlNodeRef>(
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
            Err(StructureUnmatched::TooManyElements)
        } else {
            self.pop().ok_or_else(|| StructureUnmatched::NoElementFound)
        }
    }
}

impl<N> StructureAdjuster<Option<N>> for Vec<N> {
    fn try_adjust(mut self) -> Result<Option<N>, StructureUnmatched> {
        if self.len() > 1 {
            Err(StructureUnmatched::TooManyElements)
        } else {
            Ok(self.pop())
        }
    }
}

impl FromHtml for String {
    type Source<N: HtmlNodeRef> = N;
    type Args = StringExtractionMethod;

    fn from_html<N: HtmlNodeRef>(
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

impl<T: FromHtml> FromHtml for Vec<T> {
    type Source<N: HtmlNodeRef> = Vec<T::Source<N>>;
    type Args = T::Args;

    fn from_html<N: HtmlNodeRef>(
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
    type Source<N: HtmlNodeRef> = Option<T::Source<N>>;
    type Args = T::Args;

    fn from_html<N: HtmlNodeRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        source
            .as_ref()
            .map(|n| {
                T::from_html(n, args).map_err(|e| ExtractionError::Child {
                    context: Position::Optional,
                    error: Box::new(e),
                })
            })
            .map_or(Ok(None), |v| v.map(Some))
    }
}
pub trait Selector: Sized {
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, String>;
}

// todo not force to clone?
pub trait HtmlNodeRef: Sized + Clone {
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

#[derive(Debug)]
pub enum Position {
    Index(usize),
    Selector(Option<String>),
    Optional,
    None,
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Position::Index(i) => write!(f, "[{i}]"),
            Position::Selector(select) => write!(f, "{:?}", select),
            Position::Optional => write!(f, "[optional]",),
            Position::None => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub enum StructureUnmatched {
    NoElementFound,
    TooManyElements,
    Unexpected(String),
}

impl Display for StructureUnmatched {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            StructureUnmatched::NoElementFound => write!(f, "no element found"),
            StructureUnmatched::TooManyElements => write!(f, "too many elements"),
            StructureUnmatched::Unexpected(s) => write!(f, "unexpected error: {s}"),
        }
    }
}

impl Display for ExtractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo
        match self {
            Self::StructureUnmatched(e) => {
                write!(f, "failed to get element: {e}")
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
