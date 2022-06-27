use crate::impls::from_html::TextExtractionError;
use crate::impls::from_html::TextExtractor;
use crate::never::Never;
use crate::HtmlNode;
use std::fmt::{Display, Formatter};

/// A default text extractor that extracts inner text content
impl TextExtractor for () {
    type Error = Never;

    fn extract<N: HtmlNode>(&self, source: &N) -> Result<String, Self::Error> {
        Ok(source.text_contents())
    }
}

/// An extractor that extracts the specified attribute value
impl TextExtractor for ExtractAttribute {
    type Error = AttributeNotFound;

    fn extract<N: HtmlNode>(&self, source: &N) -> Result<String, Self::Error> {
        source
            .get_attribute(&self.name)
            .map(|a| a.to_string())
            .ok_or_else(|| AttributeNotFound {
                name: self.name.clone(),
            })
    }
}

#[cfg(test)]
mod test {
    use crate::impls::from_html::FromHtmlTextError;
    use crate::impls::text_extractor::{AttributeNotFound, ExtractAttribute};
    use crate::never::Never;
    use crate::{FromHtml, HtmlNode, Selector};
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
                &ExtractAttribute {
                    name: "foo".to_string()
                }
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
                &ExtractAttribute {
                    name: "aaa".to_string()
                }
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
                &(),
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

    impl Selector for MockSelector {
        type Error = Never;

        fn parse<S: AsRef<str>>(_s: S) -> Result<Self, Self::Error> {
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

        fn get_attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str> {
            self.attributes.get(attr.as_ref()).map(|a| a.as_str())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttributeNotFound {
    pub name: String,
}

impl Display for AttributeNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "an attribute `{}` not found in the target element",
            self.name
        )
    }
}

impl TextExtractionError for AttributeNotFound {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtractAttribute {
    pub name: String,
}
