pub mod adjuster;
pub mod backend;
pub mod from_html;
pub mod from_text;
pub mod macro_utils;
pub mod never;
pub mod text_extractor;
pub mod util;

use std::fmt::{Debug, Display};

pub trait FromHtml<A>: Sized {
    type Source<N: HtmlNode>;
    type Error: FromHtmlError;

    fn from_html<N: HtmlNode>(source: &Self::Source<N>, args: &A) -> Result<Self, Self::Error>;
}

pub trait FromHtmlError: Display + Debug + 'static {}

// TODO not force to clone
pub trait HtmlNode: Sized + Clone {
    type Selector: Selector;
    fn select(&self, sel: &Self::Selector) -> Vec<Self>;
    fn text_contents(&self) -> String;
    fn get_attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str>;
}

pub trait Selector: Sized {
    type Error: ParseSelectorError;
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, Self::Error>;
}

pub trait ParseSelectorError: Display + Debug + 'static {}
