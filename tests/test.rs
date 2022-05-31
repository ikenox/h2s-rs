#![feature(generic_associated_types)]

use h2s_core::ExtractionError;

fn err() -> ExtractionError {
    ExtractionError::Unexpected("test error".to_string())
}

#[test]
fn derive_from_html() {
    use h2s::FromHtml;

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Page {
        #[h2s(attr = "lang")]
        lang: String,
        #[h2s(select = "h1.blog-title")]
        blog_title: String,
        #[h2s(select = ".articles > div", attr = "data-author")]
        article_authors: Vec<String>,
        #[h2s(select = ".articles > div > h2")]
        article_titles: Vec<String>,
        #[h2s(select = ".articles > div")]
        articles: Vec<ArticleElem>,
        #[h2s(select = "footer")]
        footer: Footer,
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct ArticleElem {
        #[h2s(select = "h2")]
        title: String,
        #[h2s(select = "p.modified-date")]
        modified_date: Option<String>,
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Footer {
        #[h2s]
        txt: String,
    }

    let html = r#"
<!DOCTYPE html>
<html lang="en">
<body>
<h1 class="blog-title">My tech blog</h1>
<div class="articles">
    <div data-author="Alice">
        <h2>article1</h2>
    </div>
    <div data-author="Bob">
        <h2>article2</h2>
        <p class="modified-date">2020-05-01</p>
    </div>
    <div data-author="Ikeno">
        <h2>article3</h2>
    </div>
</div>
<footer>this is footer</footer>
</body>
</html>
    "#;

    let res = h2s::utils::parse::<Page>(html);
    assert_eq!(
        res,
        Ok(Page {
            lang: "en".to_string(),
            blog_title: "My tech blog".to_string(),
            article_authors: vec!["Alice", "Bob", "Ikeno"]
                .iter()
                .map(|a| a.to_string())
                .collect(),
            article_titles: vec!["article1", "article2", "article3"]
                .iter()
                .map(|a| a.to_string())
                .collect(),
            articles: vec![
                ArticleElem {
                    title: "article1".to_string(),
                    modified_date: None,
                },
                ArticleElem {
                    title: "article2".to_string(),
                    modified_date: Some("2020-05-01".to_string()),
                },
                ArticleElem {
                    title: "article3".to_string(),
                    modified_date: None,
                }
            ],
            footer: Footer {
                txt: "this is footer".to_string()
            }
        })
    )
}

mod from_html {
    use crate::err;
    use h2s::FromHtml;
    use h2s_core::{ExtractAttribute, ExtractionError, Position};
    use maplit::hashmap;
    use mock::*;

    #[test]
    fn vec() {
        assert_eq!(
            Vec::<FromHtmlImpl>::from_html(&vec![MockElement::new("a"), MockElement::new("b")], ()),
            Ok(vec![FromHtmlImpl::new("a"), FromHtmlImpl::new("b")]),
            "the method is applied for each items of the vec"
        );

        assert_eq!(
            Vec::<FromHtmlImpl>::from_html(
                &vec![MockElement::new("a"), MockElement::new("error")],
                (),
            ),
            Err(ExtractionError::Child {
                context: Position::Index(1),
                error: Box::new(err())
            }),
            "returned error if one of the vec items fails to apply"
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            Option::<FromHtmlImpl>::from_html(&Some(MockElement::new("a")), ()),
            Ok(Some(FromHtmlImpl::new("a"))),
            "the method is applied for is present"
        );

        assert_eq!(
            Option::<FromHtmlImpl>::from_html::<MockElement>(&None, ()),
            Ok(None),
            "returned none if none"
        );

        assert_eq!(
            Option::<FromHtmlImpl>::from_html(&Some(MockElement::new("error")), ()),
            Err(err()),
            "returned error if failed to apply"
        );
    }

    #[test]
    fn string() {
        assert_eq!(
            String::from_html(&MockElement::new("text"), ()),
            Ok("text".to_string()),
            "inner text content will be extracted"
        );

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
        )
    }

    mod mock {
        use super::*;
        use crate::err;
        use h2s_core::{ExtractionError, HtmlElementRef, Selector};
        use std::collections::HashMap;

        #[derive(Debug, Eq, PartialEq, Clone)]
        pub struct FromHtmlImpl(String);
        impl FromHtmlImpl {
            pub fn new<S: AsRef<str>>(s: S) -> Self {
                Self(s.as_ref().to_string())
            }
        }

        impl<'a> FromHtml<'a, ()> for FromHtmlImpl {
            type Source<N: HtmlElementRef> = N;

            fn from_html<N: HtmlElementRef>(
                source: &Self::Source<N>,
                args: (),
            ) -> Result<Self, ExtractionError> {
                if source.text_contents() == "error" {
                    Err(err())
                } else {
                    Ok(FromHtmlImpl(source.text_contents()))
                }
            }
        }

        #[derive(Clone, Default)]
        pub struct MockElement {
            pub text_contents: String,
            pub attributes: HashMap<String, String>,
        }
        impl MockElement {
            pub fn new<S: AsRef<str>>(s: S) -> Self {
                Self {
                    text_contents: s.as_ref().to_string(),
                    ..Default::default()
                }
            }
        }

        pub struct SelectorMock;

        impl Selector for SelectorMock {
            fn parse<S: AsRef<str>>(s: S) -> Result<Self, String> {
                unimplemented!()
            }
        }

        impl HtmlElementRef for MockElement {
            type Selector = SelectorMock;

            fn select(&self, sel: &Self::Selector) -> Vec<Self> {
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
}
