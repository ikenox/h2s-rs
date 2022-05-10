use std::fmt::Debug;

use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

use crate::TextExtractionMethod::{Attribute, TextContent};

#[derive(Debug)]
pub enum ExtractionError {
    Unexpected(String),
    HtmlStructureUnmatched(GetElementError),
    AttributeNotFound,
    Child {
        selector: Option<String>,
        args: Box<dyn ExtractionArgs>,
        error: Box<ExtractionError>,
    },
}

#[derive(Debug)]
pub enum GetElementError {
    NoElementFound,
    StructureMismatched,
    EmptyDocument,
}

pub mod macro_utils;
pub mod types;

pub fn extract_from_html<T: FromHtml<Args = (), Source = NodeRef>>(
    s: &str,
) -> Result<T, ExtractionError> {
    let doc = kuchiki::parse_html().one(s);
    let node = T::Source::build_source_from(Foo::Single(doc))
        .map_err(|e| ExtractionError::HtmlStructureUnmatched(e))?;
    T::extract_from(&node, &())
}

pub trait FromHtml: Sized {
    type Source: ExtractionSource;
    type Args: ExtractionArgs;
    fn extract_from(source: &Self::Source, args: &Self::Args) -> Result<Self, ExtractionError>;
}

pub trait ExtractionSource: Sized + Debug {
    type Es: ExtractionSource;
    fn build_source_from(n: Foo<Self::Es>) -> Result<Self, GetElementError>;
}

impl<N: HtmlNode> ExtractionSource for N {
    type Es = N;
    fn build_source_from(n: Foo<Self::Es>) -> Result<Self, GetElementError> {
        match &n {
            Foo::List(nn) => nn
                .get(0)
                .ok_or_else(|| GetElementError::NoElementFound)
                .map(|a| a.clone()),
            Foo::Single(n) => Ok(n.clone()),
        }
    }
}

impl<S: ExtractionSource> ExtractionSource for Vec<S> {
    type Es = S;
    fn build_source_from(n: Foo<Self::Es>) -> Result<Self, GetElementError> {
        match n {
            Foo::List(nn) => Ok(nn),
            Foo::Single(n) => Err(GetElementError::StructureMismatched),
        }
    }
}
impl<S: ExtractionSource> ExtractionSource for Option<S> {
    type Es = S;
    fn build_source_from(n: Foo<Self::Es>) -> Result<Self, GetElementError> {
        Ok(match n {
            Foo::List(mut nn) => nn.into_iter().next(),
            Foo::Single(n) => Some(n),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Foo<T> {
    List(Vec<T>),
    Single(T),
}

#[derive(Debug, Clone)]
pub enum TextExtractionMethod {
    TextContent,
    Attribute(String),
}

pub trait HtmlNode: Sized + Clone + Debug {
    fn selected<S: AsRef<str>>(&self, sel: S) -> Result<Vec<Self>, SelectError>;
}

pub trait ExtractFromNode<T>: Sized {
    fn extract(&self, n: &NodeRef) -> Result<Self, ExtractionError>;
}

pub struct ArgBuilder<'a> {
    pub attr: Option<&'a str>,
}

pub trait ExtractionArgs: Debug {}
impl ExtractionArgs for () {}
impl ExtractionArgs for TextExtractionMethod {}

pub trait IntoArgs<T: ExtractionArgs> {
    fn build_args(&self) -> T;
}

impl<'a> IntoArgs<()> for ArgBuilder<'a> {
    fn build_args(&self) -> () {
        ()
    }
}

impl<'a> IntoArgs<TextExtractionMethod> for ArgBuilder<'a> {
    fn build_args(&self) -> TextExtractionMethod {
        if let Some(attr) = &self.attr {
            Attribute(attr.to_string())
        } else {
            TextContent
        }
    }
}

pub struct SelectError;
