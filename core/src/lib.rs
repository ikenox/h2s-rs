#![feature(generic_associated_types)]

mod impls;
pub mod macro_utils;
pub mod util;

use std::fmt::{Debug, Display};

pub trait FromHtml<'a, A: 'a>: Sized {
    type Source<N: HtmlNode>;

    fn from_html<N: HtmlNode>(source: &Self::Source<N>, args: A) -> Result<Self, ParseError>;
}

// TODO cannot implement into third party structs by users
pub trait FromText: Sized {
    type Err: Display + Debug + Sized + 'static;
    fn from_text(s: &str) -> Result<Self, Self::Err>;
}

// TODO not force to clone
pub trait HtmlNode: Sized + Clone {
    type Selector: Selector;
    fn select(&self, sel: &Self::Selector) -> Vec<Self>;
    fn text_contents(&self) -> String;
    fn get_attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str>;
}

pub trait Selector: Sized {
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, String>;
}

pub trait StructureAdjuster<N> {
    fn try_adjust(self) -> Result<N, StructureUnmatched>;
}

pub trait TextExtractor {
    fn extract<N: HtmlNode>(&self, source: &N) -> Result<String, TextExtractionFailed>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    Root {
        message: String,
        // TODO hold Box<dyn SomeErrorTrait>
        cause: Option<String>,
    },
    Child {
        position: Position,
        error: Box<ParseError>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Position {
    Index(usize),
    Struct {
        selector: Option<String>,
        field_name: String,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructureUnmatched(String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextExtractionFailed(String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtractAttribute(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtractInnerText;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultArg;
