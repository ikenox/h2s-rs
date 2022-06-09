#![feature(generic_associated_types)]
mod adjuster;
mod display;
mod from_html;
mod html_backend;
pub mod macro_utils;
pub mod util;

use std::fmt::Debug;

pub trait FromHtml<'a, A: 'a>: Sized {
    type Source<N: HtmlElementRef>;

    fn from_html<N: HtmlElementRef>(source: &Self::Source<N>, args: A) -> Result<Self, ParseError>;
}

// TODO not force to clone
pub trait HtmlElementRef: Sized + Clone {
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    Root(String),
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
