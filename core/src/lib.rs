use crate::TextExtractionMethod::{Attribute, TextContent};
use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::traits::TendrilSink;
use kuchiki::{ElementData, Node, NodeDataRef, NodeRef, Selectors};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct H2sError {
    node_info: (), // todo contain informative data
    detail: ExtractionError,
}

#[derive(Debug, Clone)]
pub enum ExtractionError {
    Unexpected(String),
    ElementUnmatched(GetElementError),
    AttributeNotFound,
    Child(Box<H2sError>),
}
#[derive(Debug, Clone)]
pub enum GetElementError {
    NoElementFound,
    EmptyDocument,
}

pub mod types;

pub fn extract_from_html<T: FromHtml<Args = ()>>(s: impl AsRef<str>) -> Result<T, ExtractionError> {
    let doc = kuchiki::parse_html().one(s.as_ref());
    extract_from(doc, &())
}

pub fn extract_from<A, T: FromHtml<Args = A>, N: HtmlElements>(
    node: N,
    args: &A,
) -> Result<T, ExtractionError> {
    T::extract_from(node, args)
}

pub fn select(
    node: &NodeRef,
    selector: impl AsRef<str>,
) -> Result<Select<Elements<Descendants>>, ExtractionError> {
    let selector = selector.as_ref();
    node.select(selector)
        .map_err(|_| ExtractionError::Unexpected(format!("invalid css selector: `{}`", selector)))
}

pub trait FromHtml: Sized {
    type Args;
    fn extract_from<N: HtmlElements>(select: N, args: &Self::Args)
        -> Result<Self, ExtractionError>;
}

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

pub trait IntoArgs<T> {
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
