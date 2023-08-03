use scraper::{ElementRef, Html, Node, Selector};
use std::error::Error;

use h2s_core::html::{Backend, CssSelector, HtmlDocument, HtmlElement, HtmlNode, TextNode};
use scraper::node::Text;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Scraper;

impl Backend for Scraper {
    type Document = ScraperDocument;
    type Element<'a> = ScraperHtmlElement<'a>;
    type Text<'a> = ScraperTextNode<'a>;

    fn parse_document<S>(s: S) -> Self::Document
    where
        S: AsRef<str>,
    {
        ScraperDocument(Html::parse_document(s.as_ref()))
    }
}

#[derive(Clone, Debug)]
pub struct ScraperDocument(Html);

impl HtmlDocument for ScraperDocument {
    type Element<'a> = ScraperHtmlElement<'a>;

    fn root_element(&self) -> Self::Element<'_> {
        ScraperHtmlElement(self.0.root_element())
    }
}

#[derive(Clone, Debug)]
pub struct ScraperTextNode<'a>(&'a Text);

impl<'a> TextNode for ScraperTextNode<'a> {
    fn get_text(&self) -> String {
        self.0.text.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct ScraperHtmlElement<'a>(ElementRef<'a>);

impl<'a> HtmlElement for ScraperHtmlElement<'a> {
    type Backend = Scraper;
    type Selector = ScraperCssSelector;
    type TextContents<'b> = scraper::element_ref::Text<'b>
    where
        Self:'b;

    fn select(&self, selector: &Self::Selector) -> Vec<Self> {
        self.0.select(&selector.0).map(ScraperHtmlElement).collect()
    }

    fn text_contents(&self) -> Self::TextContents<'a> {
        self.0.text()
    }

    fn attribute<S>(&self, attr: S) -> Option<&str>
    where
        S: AsRef<str>,
    {
        self.0.value().attr(attr.as_ref())
    }

    fn child_nodes(&self) -> Vec<HtmlNode<'_, Self::Backend>> {
        self.0
            .children()
            .map(|node| match node.value() {
                Node::Element(_) => {
                    HtmlNode::Element(ScraperHtmlElement(ElementRef::wrap(node).unwrap()))
                }
                Node::Text(text) => HtmlNode::Text(ScraperTextNode(text)),
                // TODO
                Node::Fragment
                | Node::Document
                | Node::Doctype(_)
                | Node::Comment(_)
                | Node::ProcessingInstruction(_) => HtmlNode::Other,
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
pub struct ScraperCssSelector(Selector);

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

// TODO generify test cases that can be applied to any backend
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
        let doc = Scraper::parse_document(
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
        let a_span = doc
            .root_element()
            .select(&CssSelector::parse("div.a > span").unwrap());
        assert_eq!(
            a_span.iter().map(|e| e.0.html()).collect::<Vec<_>>(),
            (1..=3)
                .map(|s| format!("<span>{s}</span>"))
                .collect::<Vec<_>>(),
        );

        // nested select
        let b = &doc
            .root_element()
            .select(&CssSelector::parse(".b").unwrap())[0];
        let b_span = b.select(&CssSelector::parse("span").unwrap());
        assert_eq!(b_span.len(), 1);
        assert_eq!(b_span[0].0.html(), "<span>4</span>".to_string());
    }

    #[test]
    fn text_contents() {
        let doc = Scraper::parse_document("<html><div>a<div>b</div><div>c</div></div></html>");
        assert_eq!(
            doc.root_element().text_contents().collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn get_attribute() {
        let doc = Scraper::parse_document(r#"<html><div id="foo" class="bar" /></html>"#);
        let elem = doc
            .root_element()
            .select(&CssSelector::parse("div").unwrap())[0]
            .clone();
        assert_eq!(elem.attribute("id").unwrap(), "foo");
        assert_eq!(elem.attribute("class").unwrap(), "bar");
    }

    #[test]
    fn child_nodes() {
        let doc = Scraper::parse_document("<div><div>a<div></div></div>b<div>c</div>d</div>");
        println!("{:#?}", &doc.0.tree);
        assert_eq!(
            doc.root_element()
                .select(&CssSelector::parse("div").unwrap())[0]
                .child_nodes()
                .into_iter()
                .map(|n| match n {
                    HtmlNode::Text(text) => text.get_text(),
                    HtmlNode::Element(elem) => format!(
                        "elem-{}",
                        elem.text_contents().fold("".to_string(), |a, b| a + b)
                    ),
                    _ => panic!("unexpected node type: {:?}", n),
                })
                .collect::<Vec<_>>(),
            vec!["elem-a", "b", "elem-c", "d"]
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
        );
    }
}
