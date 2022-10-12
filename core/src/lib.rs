//! A core part of h2s

pub mod backend;
pub mod display;
pub mod from_html;
pub mod from_text;
pub mod macro_utils;
pub mod mapper;
pub mod text_extractor;
pub mod transformer;
pub mod util;

use std::fmt::{Debug, Display};

/// A converter from single HTML node to single struct
pub trait FromHtml: Sized {
    type Args;
    type Error: Error;

    fn from_html<N: HtmlNode>(source: &N, args: &Self::Args) -> Result<Self, Self::Error>;
}

pub trait Error: Display + Debug + 'static {}
impl<T: Display + Debug + 'static> Error for T {}

// TODO not force to clone
/// HTML Node
pub trait HtmlNode: Sized + Clone {
    type Selector: CssSelector;
    fn select(&self, sel: &Self::Selector) -> Vec<Self>;
    fn text_contents(&self) -> String;
    fn attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str>;
}

/// CSS Selector
pub trait CssSelector: Sized {
    type Error: Error;
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, Self::Error>;
}

/// Similar with std::convert::Infallible
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Never {}
