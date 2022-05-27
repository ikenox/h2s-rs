#![feature(generic_associated_types)]

use h2s_core::{ExtractionError, FromHtml, HtmlElementRef, Selector};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct MockElement {
    select: HashMap<String, Vec<Self>>,
    text_contents: String,
    attribute: HashMap<String, String>,
}

pub struct SelectorMock(String);

impl Selector for SelectorMock {
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, String> {
        Ok(Self(s.as_ref().to_string()))
    }
}

impl HtmlElementRef for MockElement {
    type Selector = SelectorMock;

    fn select(&self, sel: &Self::Selector) -> Vec<Self> {
        self.select.get(&sel.0).unwrap().clone()
    }

    fn text_contents(&self) -> String {
        self.text_contents.clone()
    }

    fn get_attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str> {
        self.attribute.get(attr.as_ref()).map(|a| a.as_str())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FromHtmlMock(String);

impl FromHtml<Result<Self, ExtractionError>> for FromHtmlMock {
    type Source<N: HtmlElementRef> = N;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &Result<Self, ExtractionError>,
    ) -> Result<Self, ExtractionError> {
        // args.clone()
        todo!()
    }
}

fn err() -> ExtractionError {
    ExtractionError::Unexpected("error".to_string())
}

mod from_html {
    use crate::{FromHtmlMock, MockElement};
    use h2s::FromHtml;
    use h2s_core::{ExtractionError, HtmlElementRef, Position};
    use scraper::{ElementRef, Html, Selector};
    use std::collections::HashMap;
    use std::hash::Hash;

    #[test]
    fn vec() {
        assert_eq!(
            Vec::<FromHtmlMock>::from_html(
                &vec![MockElement {
                    select: Default::default(),
                    text_contents: "".to_string(),
                    attribute: Default::default()
                }],
                &(),
            ),
            Ok(["a", "b", "c"]
                .iter()
                .map(|t| FromHtmlMock(t.to_string()))
                .collect())
        );

        assert_eq!(
            Vec::<FromHtmlMock>::from_html(
                &html.root_element().select(&select("li")).collect(),
                &(),
            ),
            Err(ExtractionError::Child {
                context: Position::Index(1),
                error: Box::new(err())
            })
        );
    }

    fn select<S: AsRef<str>>(selector: S) -> Selector {
        Selector::parse(selector.as_ref()).unwrap()
    }

    // doctype html
}
