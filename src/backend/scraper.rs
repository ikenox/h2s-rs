use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

use crate::backend::Backend;
use h2s_core::html::{CssSelector, HtmlDocument, HtmlElement};
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

impl HtmlDocument for ScraperDocumentRoot {
    type Element<'a> = ScraperHtmlElement<'a>;

    fn root_element(&self) -> Self::Element<'_> {
        ScraperHtmlElement(self.0.root_element())
    }
}

#[derive(Clone, Debug)]
pub struct ScraperCssSelector(Selector);

#[derive(Clone, Debug)]
pub struct ScraperHtmlElement<'a>(ElementRef<'a>);

impl<'a> HtmlElement for ScraperHtmlElement<'a> {
    type Selector = ScraperCssSelector;

    fn select(&self, selector: &Self::Selector) -> Vec<Self> {
        self.0.select(&selector.0).map(ScraperHtmlElement).collect()
    }

    fn text_contents(&self) -> String {
        self.0.text().join("")
    }

    fn attribute<S>(&self, attr: S) -> Option<&str>
    where
        S: AsRef<str>,
    {
        self.0.value().attr(attr.as_ref())
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
            // TODO The error detail is dropped
            .map_err(|_| ParseFailed)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseFailed;

impl Error for ParseFailed {}

impl Display for ParseFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse css selector")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use h2s_core::html::{CssSelector, HtmlElement};

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
        let elem = Scraper::parse_document(
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
        let a_span = elem
            .root_element()
            .select(&CssSelector::parse("div.a > span").unwrap());
        assert_eq!(
            a_span.iter().map(|e| e.0.html()).collect::<Vec<_>>(),
            (1..=3)
                .map(|s| format!("<span>{s}</span>"))
                .collect::<Vec<_>>(),
        );

        // nested select
        let b = &elem
            .root_element()
            .select(&CssSelector::parse(".b").unwrap())[0];
        let b_span = b.select(&CssSelector::parse("span").unwrap());
        assert_eq!(b_span.len(), 1);
        assert_eq!(b_span[0].0.html(), "<span>4</span>".to_string());
    }

    #[test]
    fn text_contents() {
        let elem = Scraper::parse_document("<html><div>a<div>b</div><div>c</div></div></html>");
        assert_eq!(elem.root_element().text_contents(), "abc");
    }

    #[test]
    fn get_attribute() {
        let elem = Scraper::parse_document(r#"<html><div id="foo" class="bar" /></html>"#);
        let elem = elem
            .root_element()
            .select(&CssSelector::parse("div").unwrap())[0]
            .clone();
        assert_eq!(elem.attribute("id").unwrap(), "foo");
        assert_eq!(elem.attribute("class").unwrap(), "bar");
    }
}
