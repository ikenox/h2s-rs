use crate::FromText;
use crate::*;

impl<'a, A: TextExtractor + 'a, S: FromText> FromHtml<'a, &'a A> for S {
    type Source<N: HtmlElementRef> = N;

    fn from_html<N: HtmlElementRef>(
        source: &Self::Source<N>,
        args: &'a A,
    ) -> Result<Self, ParseError> {
        let txt = args.extract(source).map_err(|e| ParseError::Root {
            message: format!("failed to extract text: {}", e),
            cause: None,
        })?;
        S::from_text(&txt).map_err(|e| ParseError::Root {
            message: format!("failed to parse string: `{}`", txt),
            cause: Some(format!("{}", e)),
        })
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
        v.try_into().map_err(|_| ParseError::Root {
            message: "vec to array conversion is unexpectedly failed".to_string(),
            cause: None,
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

    fn ok<T>(s: &str) -> Result<String, T> {
        Ok(s.to_string())
    }

    fn err<T>(s: &str) -> Result<T, ParseError> {
        Err(ParseError::Root {
            message: s.to_string(),
            cause: None,
        })
    }

    #[test]
    fn vec() {
        assert_eq!(
            Vec::<FromHtmlImpl>::from_html::<MockElement>(
                &vec![MockElement(ok("a")), MockElement(ok("b"))],
                ()
            ),
            Ok(vec![FromHtmlImpl::new("a"), FromHtmlImpl::new("b")]),
            "the method is applied for each items of the vec"
        );

        assert_eq!(
            Vec::<FromHtmlImpl>::from_html::<MockElement>(
                &vec![MockElement(Ok("a".into())), MockElement(err("err!"))],
                (),
            ),
            Err(ParseError::Child {
                position: Position::Index(1),
                error: Box::new(err::<()>("err!").unwrap_err())
            }),
            "returned error if one of the vec items fails to apply"
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            Option::<FromHtmlImpl>::from_html::<MockElement>(&Some(MockElement(ok("ok!"))), ()),
            Ok(Some(FromHtmlImpl::new("ok!"))),
            "the method is applied for is present"
        );

        assert_eq!(
            Option::<FromHtmlImpl>::from_html::<MockElement>(&None, ()),
            Ok(None),
            "returned none if none"
        );

        assert_eq!(
            Option::<FromHtmlImpl>::from_html::<MockElement>(&Some(MockElement(err("err!"))), ()),
            err("err!"),
            "returned error if failed to apply"
        );
    }

    #[test]
    fn from_text() {
        struct MockExtractor {}
        impl TextExtractor for MockExtractor {
            fn extract<N: HtmlElementRef>(
                &self,
                _source: &N,
            ) -> Result<String, TextExtractionFailed> {
                ok("ok!")
            }
        }

        assert_eq!(
            String::from_html(&MockElement(ok("")), &MockExtractor {}),
            ok("ok!"),
            "the extraction result is returned",
        );
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct FromHtmlImpl(String);

    impl FromHtmlImpl {
        pub fn new<S: AsRef<str>>(s: S) -> Self {
            Self(s.as_ref().to_string())
        }
    }

    impl<'a> FromHtml<'a, ()> for FromHtmlImpl {
        type Source<N: HtmlElementRef> = MockElement;

        fn from_html<N: HtmlElementRef>(
            source: &MockElement,
            _args: (),
        ) -> Result<Self, ParseError> {
            source.clone().0.map(FromHtmlImpl)
        }
    }

    #[derive(Clone)]
    pub struct MockElement(Result<String, ParseError>);
    pub struct MockSelector;

    impl Selector for MockSelector {
        fn parse<S: AsRef<str>>(s: S) -> Result<Self, String> {
            todo!()
        }
    }

    impl HtmlElementRef for MockElement {
        type Selector = MockSelector;

        fn select(&self, sel: &Self::Selector) -> Vec<Self> {
            todo!()
        }

        fn text_contents(&self) -> String {
            todo!()
        }

        fn get_attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str> {
            todo!()
        }
    }
}
