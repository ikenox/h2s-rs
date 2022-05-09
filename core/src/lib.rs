use std::any::Any;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::traits::TendrilSink;
use kuchiki::{ElementData, NodeDataRef, NodeRef, Selectors};

use crate::TextExtractionMethod::{Attribute, TextContent};

#[derive(Debug, Clone)]
pub enum ExtractionError {
    Unexpected(String),
    HtmlStructureUnmatched(GetElementError),
    AttributeNotFound,
    Child {
        selector: Option<String>,
        args: Rc<dyn ExtractionArgs>,
        error: Box<ExtractionError>,
    },
}

#[derive(Debug, Clone)]
pub enum GetElementError {
    NoElementFound,
    EmptyDocument,
}

pub mod macro_utils;
pub mod types;

pub fn extract_from_html<T: FromHtml<Args = ()>>(s: impl AsRef<str>) -> Result<T, ExtractionError> {
    let doc = kuchiki::parse_html()
        .one(s.as_ref())
        .first_child()
        .ok_or_else(|| ExtractionError::HtmlStructureUnmatched(GetElementError::EmptyDocument))?;
    extract_from(doc, &())
}

pub fn extract_from<A, T: FromHtml<Args = A>, N: HtmlElements>(
    node: N,
    args: &A,
) -> Result<T, ExtractionError> {
    T::extract_from(node, args)
}

pub trait FromHtml: Sized {
    type Args: ExtractionArgs;
    fn extract_from<N: HtmlElements>(select: N, args: &Self::Args)
        -> Result<Self, ExtractionError>;
}

#[derive(Debug, Clone)]
pub enum TextExtractionMethod {
    TextContent,
    Attribute(String),
}

pub trait HtmlElements {
    fn exactly_one_or_none(self) -> Result<Option<NodeDataRef<ElementData>>, GetElementError>;
    fn get_exactly_one(self) -> Result<NodeDataRef<ElementData>, GetElementError>;
    fn list(self) -> Vec<NodeDataRef<ElementData>>;
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
