use crate::Never;
use crate::{Error, HtmlNode};
use std::fmt::Debug;

pub trait TextExtractor {
    type Error: Error;
    fn extract<N>(&self, source: &N) -> Result<String, Self::Error>
    where
        N: HtmlNode;
}

pub mod impls {
    use super::*;

    /// A default text extractor that extracts inner text content
    impl TextExtractor for () {
        type Error = Never;

        fn extract<N>(&self, source: &N) -> Result<String, Self::Error>
        where
            N: HtmlNode,
        {
            Ok(source.text_contents())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ExtractAttribute {
        pub name: String,
    }

    /// An extractor that extracts the specified attribute value
    impl TextExtractor for ExtractAttribute {
        type Error = AttributeNotFound;

        fn extract<N>(&self, source: &N) -> Result<String, Self::Error>
        where
            N: HtmlNode,
        {
            source
                .attribute(&self.name)
                .map(|a| a.to_string())
                .ok_or_else(|| AttributeNotFound {
                    name: self.name.clone(),
                })
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct AttributeNotFound {
        pub name: String,
    }
}

#[cfg(test)]
mod test {
    use crate::from_html::{ExtractionType, FromHtmlTextError};
    use crate::text_extractor::impls::AttributeNotFound;
    use crate::Never;
    use crate::{CssSelector, FromHtml, HtmlNode};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn attribute() {
        assert_eq!(
            String::from_html(
                &MockElement {
                    attributes: hashmap! {
                        "foo".to_string() => "bar".to_string(),
                    },
                    ..Default::default()
                },
                &ExtractionType::Attribute("foo".to_string())
            ),
            Ok("bar".to_string()),
            "correct attribute value will be extracted"
        );

        assert_eq!(
            String::from_html(
                &MockElement {
                    attributes: hashmap! {
                        "foo".to_string() => "bar".to_string(),
                    },
                    ..Default::default()
                },
                &ExtractionType::Attribute("aaa".to_string())
            ),
            Err(FromHtmlTextError::ExtractionFailed(AttributeNotFound {
                name: "aaa".to_string()
            })),
            "error when element doesn't have the specified attribute"
        );
    }

    #[test]
    fn inner_text() {
        assert_eq!(
            String::from_html(
                &MockElement {
                    text_contents: "foo".to_string(),
                    ..Default::default()
                },
                &ExtractionType::Text,
            ),
            Ok("foo".to_string()),
        );
    }

    #[derive(Clone, Default)]
    pub struct MockElement {
        pub text_contents: String,
        pub attributes: HashMap<String, String>,
    }

    pub struct MockSelector;

    impl CssSelector for MockSelector {
        type Error = Never;

        fn parse<S>(_s: S) -> Result<Self, Self::Error>
        where
            S: AsRef<str>,
        {
            unimplemented!()
        }
    }

    impl HtmlNode for MockElement {
        type Selector = MockSelector;

        fn select(&self, _sel: &Self::Selector) -> Vec<Self> {
            unimplemented!()
        }

        fn text_contents(&self) -> String {
            self.text_contents.clone()
        }

        fn attribute<S>(&self, attr: S) -> Option<&str>
        where
            S: AsRef<str>,
        {
            self.attributes.get(attr.as_ref()).map(|a| a.as_str())
        }
    }
}
