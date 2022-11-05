use h2s_core::{CssSelector, HtmlNode};
use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};

use crate::backend::{Backend, DocumentRoot};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Scraper;
impl Backend for Scraper {
    type Root = ScraperDocumentRoot;

    fn parse_document<S>(s: S) -> Self::Root
    where
        S: AsRef<str>,
    {
        ScraperDocumentRoot(Html::parse_document(s.as_ref()))
    }
}

#[derive(Clone, Debug)]
pub struct ScraperDocumentRoot(Html);
impl DocumentRoot for ScraperDocumentRoot {
    type HtmlNode<'a> = ScraperHtmlNode<'a>;

    fn root_element(&self) -> Self::HtmlNode<'_> {
        ScraperHtmlNode::ElementRef(self.0.root_element())
    }
}

#[derive(Clone, Debug)]
pub struct ScraperCssSelector(Selector);

#[derive(Clone, Debug)]
pub enum ScraperHtmlNode<'a> {
    ElementRef(ElementRef<'a>),
}

impl<'a> ScraperHtmlNode<'a> {
    fn as_element_ref(&self) -> ElementRef<'a> {
        match self {
            ScraperHtmlNode::ElementRef(elem) => *elem,
        }
    }
}

impl<'a> HtmlNode for ScraperHtmlNode<'a> {
    type Selector = ScraperCssSelector;

    fn select(&self, selector: &Self::Selector) -> Vec<Self> {
        self.as_element_ref()
            .select(&selector.0)
            .map(ScraperHtmlNode::<'a>::ElementRef)
            .collect()
    }

    fn text_contents(&self) -> String {
        self.as_element_ref().text().join("")
    }

    fn attribute<S>(&self, attr: S) -> Option<&str>
    where
        S: AsRef<str>,
    {
        self.as_element_ref().value().attr(attr.as_ref())
    }
}

impl CssSelector for ScraperCssSelector {
    type Error = ParseFailed;

    fn parse<S>(s: S) -> Result<Self, ParseFailed>
    where
        S: AsRef<str>,
    {
        Selector::parse(s.as_ref())
            .map(ScraperCssSelector)
            // FIXME The error detail is dropped
            .map_err(|_| ParseFailed)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseFailed;

impl std::error::Error for ParseFailed {}

impl Display for ParseFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse css selector")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{CssSelector, HtmlNode};

    #[test]
    fn selector() {
        assert!(ScraperCssSelector::parse("div > .a").is_ok());

        assert_eq!(
            ScraperCssSelector::parse(":invalid:").unwrap_err(),
            ParseFailed
        );
    }

    #[test]
    fn select() {
        let node = Scraper::parse_document(
            r#"
<!DOCTYPE html>
<html>
<body>
<div class="a">
    <span>1</span>
    <span>2</span>
    <span>3</span>
</div>

<div class="b">
    <span>4</span>
</div>

<span>5</span>
</body>
</html>
        "#,
        );
        let a_span = node
            .root_element()
            .select(&CssSelector::parse("div.a > span").unwrap());
        assert_eq!(
            a_span
                .iter()
                .map(|e| e.as_element_ref().html())
                .collect::<Vec<_>>(),
            (1..=3)
                .map(|s| format!("<span>{s}</span>"))
                .collect::<Vec<_>>(),
        );

        // nested select
        let b = &node
            .root_element()
            .select(&CssSelector::parse(".b").unwrap())[0];
        let b_span = b.select(&CssSelector::parse("span").unwrap());
        assert_eq!(b_span.len(), 1);
        assert_eq!(
            b_span[0].as_element_ref().html(),
            "<span>4</span>".to_string()
        );
    }

    #[test]
    fn text_contents() {
        let node = Scraper::parse_document("<html><div>a<div>b</div><div>c</div></div></html>");
        assert_eq!(node.root_element().text_contents(), "abc");
    }

    #[test]
    fn get_attribute() {
        let node = Scraper::parse_document(r#"<html><div id="foo" class="bar" /></html>"#);
        let elem = node
            .root_element()
            .select(&CssSelector::parse("div").unwrap())[0]
            .clone();
        assert_eq!(elem.attribute("id").unwrap(), "foo");
        assert_eq!(elem.attribute("class").unwrap(), "bar");
    }
}
