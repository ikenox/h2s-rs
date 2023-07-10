//! A core part of h2s

use std::fmt::Debug;

pub mod display;
pub mod error;
pub mod field_value;
pub mod from_html;
pub mod from_text;
pub mod macro_utils;
pub mod text_extractor;
pub mod transformer;

/// A converter from single HTML node to single struct
pub trait FromHtml: Sized {
    type Args;
    type Error: Error;

    fn from_html<N>(source: &N, args: &Self::Args) -> Result<Self, Self::Error>
    where
        N: HtmlNode;
}

/// HTML document
pub trait HtmlDocument {
    type HtmlNode<'a>: HtmlNode
    where
        Self: 'a;

    fn root_element(&self) -> Self::HtmlNode<'_>;
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

/// A field value of FromHtml-deriving struct
pub trait FieldValue: Sized {
    type Inner: FromHtml;
    type Structure<U>;
    type Error<E: Error>: Error;

    /// This method converts from `Structure<A>` to `Result<Structure<T>>`
    /// It works like a `traverse` of functional programming language
    fn try_traverse_from<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
    where
        F: Fn(A) -> Result<Self::Inner, E>,
        E: Error;
}

/// Similar with std::convert::Infallible
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Never {}

/// Common error trait
pub trait Error: std::error::Error + Sync + Send + 'static {}

impl<T> Error for T where T: std::error::Error + Sync + Send + 'static {}
