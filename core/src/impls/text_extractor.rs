use crate::ExtractAttribute;
use crate::{HtmlNode, TextExtractionFailed, TextExtractor};

/**
A default text extractor that extracts inner text content
*/
impl TextExtractor for () {
    fn extract<N: HtmlNode>(&self, source: &N) -> Result<String, TextExtractionFailed> {
        Ok(source.text_contents())
    }
}

/**
An extractor that extracts the specified attribute value
 */
impl TextExtractor for ExtractAttribute {
    fn extract<N: HtmlNode>(&self, source: &N) -> Result<String, TextExtractionFailed> {
        source
            .get_attribute(&self.0)
            .map(|a| a.to_string())
            .ok_or_else(|| TextExtractionFailed(format!("attribute `{}` not found", self.0)))
    }
}

#[cfg(test)]
mod test {
    use crate::{ExtractAttribute, FromHtml, HtmlNode, ParseError, Selector};
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
                &ExtractAttribute("foo".to_string())
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
                &ExtractAttribute("aaa".to_string())
            ),
            Err(ParseError::Root {
                message: "failed to extract text: attribute `aaa` not found".to_string(),
                cause: None
            }),
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
        fn parse<S: AsRef<str>>(_s: S) -> Result<Self, String> {
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
