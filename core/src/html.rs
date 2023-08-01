use std::error::Error;
use std::fmt::Debug;

/// HTML document
pub trait HtmlDocument: Debug {
    type Element<'a>: HtmlElement
    where
        Self: 'a;

    fn root_element(&self) -> Self::Element<'_>;
}

/// HTML Element
pub trait HtmlElement: Sized + Debug + Clone {
    type Backend: Backend;
    type Selector: CssSelector;

    fn select(&self, selector: &Self::Selector) -> Vec<Self>;
    // TODO remove this method
    fn text_contents(&self) -> String;
    fn attribute<S>(&self, attr: S) -> Option<&str>
    where
        S: AsRef<str>;
    fn child_nodes(&self) -> Vec<HtmlNode<'_, Self::Backend>>;
}

/// CSS Selector
pub trait CssSelector: Sized {
    type Error: Error;
    fn parse<S>(s: S) -> Result<Self, Self::Error>
    where
        S: AsRef<str>;
}

#[derive(Debug, Clone)]
pub enum HtmlNode<'a, B>
where
    B: Backend,
{
    Document(B::Document),
    Element(B::Element<'a>),
    Text(B::Text<'a>),
    // TODO
    Other,
}

pub trait TextNode: Debug {
    fn get_text(&self) -> String;
}

pub trait Backend: Debug {
    type Document: HtmlDocument;
    type Element<'a>: HtmlElement;
    type Text<'a>: TextNode;
    fn parse_document<S>(s: S) -> Self::Document
    where
        S: AsRef<str>;
}
