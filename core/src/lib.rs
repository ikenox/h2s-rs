#![feature(generic_associated_types)]
mod adjuster;
mod display;
mod from_html;
mod html_backend;
pub mod macro_utils;
pub mod utils;

use std::fmt::Debug;

pub trait FromHtml<'a, A: 'a>: Sized {
    type Source<N: HtmlElementRef>;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: A,
    ) -> Result<Self, ExtractionError>;
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

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Index(usize),
    Struct {
        selector: Option<String>,
        field_name: String,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructureUnmatched(String);

pub enum StringExtractionMethod {
    Text,
    Attribute(String),
}

pub trait StructureAdjuster<N> {
    fn try_adjust(self) -> Result<N, StructureUnmatched>;
}

pub struct ExtractAttribute(pub String);

pub trait Selector: Sized {
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, String>;
}
