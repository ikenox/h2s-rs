use crate::{HtmlNode, ParseSelectorError, Selector};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

impl Selector for scraper::Selector {
    type Error = ParseFailed;

    fn parse<S: AsRef<str>>(s: S) -> Result<Self, ParseFailed> {
        scraper::Selector::parse(s.as_ref()).map_err(|_| ParseFailed)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseFailed;

impl ParseSelectorError for ParseFailed {}

impl Display for ParseFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse css selector")
    }
}

impl<'a> HtmlNode for scraper::ElementRef<'a> {
    type Selector = scraper::Selector;

    fn select(&self, sel: &Self::Selector) -> Vec<Self> {
        scraper::ElementRef::<'a>::select(self, sel).collect()
    }

    fn text_contents(&self) -> String {
        self.text().join(" ")
    }

    fn get_attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str> {
        self.value().attr(attr.as_ref())
    }
}

#[cfg(test)]
mod test {
    use crate::backend::scraper::ParseFailed;
    use crate::{HtmlNode, Selector};

    #[test]
    fn selector() {
        assert_eq!(
            <scraper::Selector as super::Selector>::parse("div > .a").unwrap(),
            scraper::Selector::parse("div > .a").unwrap(),
        );

        assert_eq!(
            <scraper::Selector as super::Selector>::parse(":invalid:"),
            Err(ParseFailed),
        );
    }

    #[test]
    fn select() {
        let doc = scraper::Html::parse_document(
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
        let elem = doc.root_element();
        let a_span = HtmlNode::select(&elem, &Selector::parse("div.a > span").unwrap());
        assert_eq!(
            a_span.iter().map(|e| e.html()).collect::<Vec<_>>(),
            (1..=3)
                .map(|s| format!("<span>{s}</span>"))
                .collect::<Vec<_>>(),
        );

        // nested select
        let b = HtmlNode::select(&elem, &Selector::parse(".b").unwrap())[0];
        let b_span = HtmlNode::select(&b, &Selector::parse("span").unwrap());
        assert_eq!(b_span.len(), 1);
        assert_eq!(b_span[0].html(), "<span>4</span>".to_string());
    }

    #[test]
    fn text_contents() {
        let doc =
            scraper::Html::parse_fragment("<html><div>a<div>b</div><div>c</div></div></html>");
        let elem = doc.root_element();
        assert_eq!(elem.text_contents(), "a b c");
    }

    #[test]
    fn get_attribute() {
        let doc = scraper::Html::parse_fragment(r#"<html><div id="foo" class="bar" /></html>"#);
        let elem = doc
            .root_element()
            .select(&Selector::parse("div").unwrap())
            .next()
            .unwrap();
        println!("{}", elem.html());
        assert_eq!(elem.get_attribute("id").unwrap(), "foo");
        assert_eq!(elem.get_attribute("class").unwrap(), "bar");
    }
}
