use crate::*;

impl<'a> FromHtml<'a, ()> for String {
    type Source<N: HtmlElementRef> = N;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        _args: (),
    ) -> Result<Self, ParseError> {
        Ok(source.text_contents())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtractAttribute(pub String);

impl<'a> FromHtml<'a, &'a ExtractAttribute> for String {
    type Source<N: HtmlElementRef> = N;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &'a ExtractAttribute,
    ) -> Result<Self, ParseError> {
        source
            .get_attribute(&args.0)
            .map(|s| s.to_string())
            .ok_or_else(|| ParseError::Root(format!("attribute `{}` not found", args.0)))
    }
}

impl<'a, B: Copy + 'a, T: FromHtml<'a, B>, const A: usize> FromHtml<'a, B> for [T; A] {
    type Source<N: HtmlElementRef> = [T::Source<N>; A];

    fn from_html<N: HtmlElementRef>(source: &Self::Source<N>, args: B) -> Result<Self, ParseError> {
        let v = source
            .iter()
            .enumerate()
            .map(|(i, n)| {
                T::from_html(n, args).map_err(|e| ParseError::Child {
                    position: Position::Index(i),
                    error: Box::new(e),
                })
            })
            .fold(Ok(vec![]), |acc, res| {
                acc.and_then(|mut list| {
                    res.map(|val| {
                        list.push(val);
                        list
                    })
                })
            })?;

        // this conversion should never fail because it has been already checked at build time
        v.try_into().map_err(|_| {
            ParseError::Root("vec to array conversion is unexpectedly failed".to_string())
        })
    }
}

impl<'a, A: Copy + 'a, T: FromHtml<'a, A>> FromHtml<'a, A> for Vec<T> {
    type Source<N: HtmlElementRef> = Vec<T::Source<N>>;

    fn from_html<N: HtmlElementRef>(source: &Self::Source<N>, args: A) -> Result<Self, ParseError> {
        source
            .iter()
            .enumerate()
            .map(|(i, n)| {
                T::from_html(n, args).map_err(|e| ParseError::Child {
                    position: Position::Index(i),
                    error: Box::new(e),
                })
            })
            // unwrapping results
            .fold(Ok(vec![]), |acc, res| {
                acc.and_then(|mut list| {
                    res.map(|val| {
                        list.push(val);
                        list
                    })
                })
            })
    }
}

impl<'a, A: 'a, T: FromHtml<'a, A>> FromHtml<'a, A> for Option<T> {
    type Source<N: HtmlElementRef> = Option<T::Source<N>>;

    fn from_html<N: HtmlElementRef>(source: &Self::Source<N>, args: A) -> Result<Self, ParseError> {
        source
            .as_ref()
            .map(|n| T::from_html(n, args))
            .map_or(Ok(None), |v| v.map(Some))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;
    use mock::*;

    fn err() -> ParseError {
        ParseError::Root("test error".to_string())
    }

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
            Err(ParseError::Child {
                position: Position::Index(1),
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
    fn string_inner_text() {
        assert_eq!(
            String::from_html(&MockElement::new("text"), ()),
            Ok("text".to_string()),
            "inner text content will be extracted"
        );
    }

    #[test]
    fn string_attribute() {
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
            Err(ParseError::Root("attribute `aaa` not found".to_string())),
            "error when element doesn't have the specified attribute"
        );
    }

    mod mock {
        use super::*;
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
                _args: (),
            ) -> Result<Self, ParseError> {
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
            fn parse<S: AsRef<str>>(_s: S) -> Result<Self, String> {
                unimplemented!()
            }
        }

        impl HtmlElementRef for MockElement {
            type Selector = SelectorMock;

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
}
