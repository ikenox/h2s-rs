//! A core part of h2s

pub mod display;
mod error;
pub mod from_html;
pub mod from_text;
pub mod functional;
pub mod macro_utils;
pub mod mapper;
pub mod text_extractor;
pub mod transformer;

use std::fmt::Debug;

/// A converter from single HTML node to single struct
pub trait FromHtml: Sized {
    type Args;
    type Error: Error;

    fn from_html<N>(source: &N, args: &Self::Args) -> Result<Self, Self::Error>
    where
        N: HtmlNode;
}

// TODO not force to clone
/// HTML Node
pub trait HtmlNode: Sized + Clone {
    type Selector: CssSelector;

    fn select(&self, selector: &Self::Selector) -> Vec<Self>;
    fn text_contents(&self) -> String;
    fn attribute<S>(&self, attr: S) -> Option<&str>
    where
        S: AsRef<str>;
}

/// CSS Selector
pub trait CssSelector: Sized {
    type Error: Error;
    fn parse<S>(s: S) -> Result<Self, Self::Error>
    where
        S: AsRef<str>;
}

/// Similar with std::convert::Infallible
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Never {}

/// Common error trait
pub trait Error: std::error::Error + Sync + Send + 'static {}

impl<T> Error for T where T: std::error::Error + Sync + Send + 'static {}
