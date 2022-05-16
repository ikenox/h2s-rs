#![feature(generic_associated_types)]
pub mod utils;

use std::any::Any;
use std::fmt::{format, Debug, Display, Formatter};
use std::marker::PhantomData;

use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

// ==========
pub fn select<N: HtmlNodeRef>(input: &N, selector: &String) -> Result<Vec<N>, ExtractionError> {
    input
        .selected(selector)
        .map_err(|_| ExtractionError::Unexpected(format!("select failed")))
}

pub fn extract<T: Nodes, S: SourceAdjuster<T>, E: Extractor>(
    source: S,
    extractor: E,
) -> Result<T::Structure<E::Output>, ExtractionError> {
    source
        .adjust_to()
        .map_err(|e| ExtractionError::StructureUnmatched(e))
        .and_then(|a| a.apply(extractor))
}
// ==========

pub trait Nodes {
    type Structure<T>;
    fn apply<E: Extractor>(self, e: E) -> Result<Self::Structure<E::Output>, ExtractionError>;
}

impl<N: HtmlNodeRef> Nodes for N {
    type Structure<A> = A;

    fn apply<E: Extractor>(self, e: E) -> Result<Self::Structure<E::Output>, ExtractionError> {
        e.extract(&self)
    }
}

impl<T: HtmlNodeRef> Nodes for Vec<T> {
    type Structure<A> = Vec<A>;
    fn apply<E: Extractor>(self, e: E) -> Result<Self::Structure<E::Output>, ExtractionError> {
        self.into_iter()
            .enumerate()
            .map(|(i, a)| {
                e.extract(&a).map_err(|a| ExtractionError::Child {
                    context: format!("[{i}]"),
                    error: Box::new(a),
                })
            })
            .fold(Ok(vec![]), |acc, item| {
                acc.and_then(|vv| item.map(|i| (vv, i))).map(|(mut vv, v)| {
                    vv.push(v);
                    vv
                })
            })
    }
}

impl<T: HtmlNodeRef> Nodes for Option<T> {
    type Structure<A> = Option<A>;
    fn apply<E: Extractor>(self, e: E) -> Result<Self::Structure<E::Output>, ExtractionError> {
        match self.as_ref().map(|n| e.extract(n)) {
            Some(Ok(e)) => Ok(Some(e)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }
}

// todo not force to clone?
pub trait HtmlNodeRef: Sized + Clone {
    fn selected<S: AsRef<str>>(&self, sel: S) -> Result<Vec<Self>, SelectError>;
    fn text_contents(&self) -> String;
    fn get_attribute<S: AsRef<str>>(&self, sel: S) -> Option<String>;
}

pub trait SourceAdjuster<T: Nodes>: Nodes {
    fn adjust_to(self) -> Result<T, StructureUnmatched>;
}

#[derive(Debug)]
pub enum ExtractionError {
    Unexpected(String),
    StructureUnmatched(StructureUnmatched),
    AttributeNotFound,
    Child {
        context: String,
        error: Box<ExtractionError>,
    },
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
            Self::AttributeNotFound => {
                write!(f, "attribute not found")
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

#[derive(Debug, Clone)]
pub struct SelectError;

pub struct TextContentExtractor;
pub struct AttributeExtractor {
    pub attr: String,
}
pub struct StructExtractor<T: FromHtml>(PhantomData<T>);
pub trait FromHtml: Sized {
    fn from_html<N: HtmlNodeRef>(input: &N) -> Result<Self, ExtractionError>;
}

impl<T: FromHtml> Extractor for StructExtractor<T> {
    type Output = T;

    fn extract<N: HtmlNodeRef>(&self, input: &N) -> Result<Self::Output, ExtractionError> {
        T::from_html(input)
    }
}

impl<T: FromHtml> StructExtractor<T> {
    pub fn new() -> Self {
        StructExtractor(PhantomData)
    }
}

impl Extractor for TextContentExtractor {
    type Output = String;

    fn extract<N: HtmlNodeRef>(&self, input: &N) -> Result<Self::Output, ExtractionError> {
        // todo consider that should we return error when empty
        Ok(input.text_contents())
    }
}
impl Extractor for AttributeExtractor {
    type Output = String;

    fn extract<N: HtmlNodeRef>(&self, input: &N) -> Result<Self::Output, ExtractionError> {
        input
            .get_attribute(&self.attr)
            .ok_or_else(|| ExtractionError::AttributeNotFound)
    }
}

impl HtmlNodeRef for NodeRef {
    fn selected<S: AsRef<str>>(&self, sel: S) -> Result<Vec<Self>, SelectError> {
        Ok(self
            .select(sel.as_ref())
            .map_err(|_| SelectError)?
            .into_iter()
            .map(|a| a.as_node().clone())
            .collect())
    }

    fn text_contents(&self) -> String {
        self.text_contents()
    }

    fn get_attribute<S: AsRef<str>>(&self, sel: S) -> Option<String> {
        self.as_element().and_then(|e| {
            e.attributes
                .borrow()
                .get(sel.as_ref())
                .map(|a| a.to_string())
        })
    }
}

pub trait Extractor {
    type Output;
    fn extract<N: HtmlNodeRef>(&self, input: &N) -> Result<Self::Output, ExtractionError>;
}

pub struct SelectedNodes<N: HtmlNodeRef> {
    pub node: N,
    pub selector: String,
}

impl<N: HtmlNodeRef> SourceAdjuster<N> for N {
    fn adjust_to(self) -> Result<N, StructureUnmatched> {
        Ok(self)
    }
}

impl<N: HtmlNodeRef> SourceAdjuster<N> for Vec<N> {
    fn adjust_to(mut self) -> Result<N, StructureUnmatched> {
        if self.len() > 1 {
            Err(StructureUnmatched::TooManyElements)
        } else {
            self.pop().ok_or_else(|| StructureUnmatched::NoElementFound)
        }
    }
}
impl<N: HtmlNodeRef> SourceAdjuster<Option<N>> for Vec<N> {
    fn adjust_to(mut self) -> Result<Option<N>, StructureUnmatched> {
        if self.len() > 1 {
            Err(StructureUnmatched::TooManyElements)
        } else {
            Ok(self.pop())
        }
    }
}

impl<N: HtmlNodeRef> SourceAdjuster<Vec<N>> for Vec<N> {
    fn adjust_to(self) -> Result<Vec<N>, StructureUnmatched> {
        Ok(self)
    }
}
