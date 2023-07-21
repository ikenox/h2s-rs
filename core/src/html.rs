use std::error::Error;

/// HTML document
pub trait HtmlDocument {
    type Element<'a>: HtmlElement
    where
        Self: 'a;

    fn root_element(&self) -> Self::Element<'_>;
}

// TODO remove Clone constraint
/// HTML Element
pub trait HtmlElement: Sized + Clone {
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
