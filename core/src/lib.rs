use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::traits::TendrilSink;
use kuchiki::{Node, NodeRef, Selectors};

#[derive(Debug, Clone)]
pub enum H2sError {
    Unexpected(String),
    NotMatched(NotMatchedDetail),
}

#[derive(Debug, Clone)]
pub enum NotMatchedDetail {
    MissingElement,
}

pub mod types;

pub trait H2s {
    fn parse(node: &NodeRef) -> Result<Self, H2sError>
    where
        Self: Sized;
}

pub struct Extractor {
    pub selector: String,
}

impl Extractor {
    pub fn extract<T: ExtractFrom>(&self, node: &NodeRef) -> Result<T, H2sError> {
        T::extract_from(
            node.select(&self.selector)
                .map_err(|e| H2sError::Unexpected(format!("invalid css selector")))?,
        )
    }
}

pub trait ExtractFrom: Sized {
    fn extract_from(select: Select<Elements<Descendants>>) -> Result<Self, H2sError>;
}

pub fn parse<T: H2s>(html: impl AsRef<str>) -> Result<T, H2sError> {
    let document = kuchiki::parse_html().one(html.as_ref());
    T::parse(&document)
}
