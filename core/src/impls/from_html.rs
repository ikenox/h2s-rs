use crate::impls::adjuster::AdjustStructureError;
use crate::impls::from_text::{FromText, FromTextError};
use crate::impls::text_extractor::{TextExtractionError, TextExtractor};
use crate::*;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructFieldError<A: AdjustStructureError, B: FromHtmlError> {
    pub selector: Option<String>,
    pub field_name: String,
    pub error: StructErrorCause<A, B>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StructErrorCause<A: AdjustStructureError, B: FromHtmlError> {
    StructureUnmatched(A),
    ParseError(B),
}

impl<A: TextExtractor, S: FromText> FromHtml<A> for S {
    type Source<N: HtmlNode> = N;
    type Error = FromHtmlTextError<A::Error, S::Error>;

    fn from_html<N: HtmlNode>(source: &Self::Source<N>, args: &A) -> Result<Self, Self::Error> {
        let txt = args
            .extract(source)
            .map_err(FromHtmlTextError::ExtractionFailed)?;
        S::from_text(&txt).map_err(FromHtmlTextError::TextParseError)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FromHtmlTextError<A: TextExtractionError, B: FromTextError> {
    ExtractionFailed(A),
    TextParseError(B),
}

impl<A, T: FromHtml<A>, const M: usize> FromHtml<A> for [T; M] {
    type Source<N: HtmlNode> = [T::Source<N>; M];
    type Error = ListElementError<T::Error>;

    fn from_html<N: HtmlNode>(source: &Self::Source<N>, args: &A) -> Result<Self, Self::Error> {
        let v = source
            .iter()
            .enumerate()
            .map(|(i, n)| {
                T::from_html(n, args).map_err(|e| ListElementError { index: i, error: e })
            })
            .fold(Ok(vec![]), |acc, res| {
                acc.and_then(|mut list| {
                    res.map(|val| {
                        list.push(val);
                        list
                    })
                })
            })?;

        // this conversion should never fail
        // TODO avoid unwrap
        Ok(v.try_into().map_err(|_| "").unwrap())
    }
}

impl<A, T: FromHtml<A>> FromHtml<A> for Vec<T> {
    type Source<N: HtmlNode> = Vec<T::Source<N>>;
    type Error = ListElementError<T::Error>;

    fn from_html<N: HtmlNode>(source: &Self::Source<N>, args: &A) -> Result<Self, Self::Error> {
        source
            .iter()
            .enumerate()
            .map(|(i, n)| {
                T::from_html(n, args).map_err(|e| ListElementError { index: i, error: e })
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListElementError<E: FromHtmlError> {
    pub index: usize,
    pub error: E,
}

impl<A, T: FromHtml<A>> FromHtml<A> for Option<T> {
    type Source<N: HtmlNode> = Option<T::Source<N>>;
    type Error = T::Error;

    fn from_html<N: HtmlNode>(source: &Self::Source<N>, args: &A) -> Result<Self, Self::Error> {
        source
            .as_ref()
            .map(|n| T::from_html(n, args))
            .map_or(Ok(None), |v| v.map(Some))
    }
}

mod display {
    use super::*;

    impl<A: AdjustStructureError, B: FromHtmlError> Display for StructFieldError<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "({}){}: ",
                self.field_name,
                self.selector
                    .as_ref()
                    .map(|s| format!(" {s}"))
                    .unwrap_or("".into()),
            )?;
            match &self.error {
                StructErrorCause::StructureUnmatched(e) => write!(f, "structure unmatched: {e}"),
                StructErrorCause::ParseError(e) => write!(f, "failed to parse: {e}"),
            }
        }
    }

    impl<A: TextExtractionError, B: FromTextError> Display for FromHtmlTextError<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "failed to convert text into the value: ")?;
            match &self {
                FromHtmlTextError::ExtractionFailed(e) => {
                    write!(f, "{e}")
                }
                FromHtmlTextError::TextParseError(e) => {
                    write!(f, "{e}")
                }
            }
        }
    }

    impl<E: FromHtmlError> Display for ListElementError<E> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]: {}", self.index, self.error)
        }
    }
}

mod error {
    use super::*;

    impl<A: AdjustStructureError, B: FromHtmlError> FromHtmlError for StructFieldError<A, B> {}
    impl<A: TextExtractionError, B: FromTextError> FromHtmlError for FromHtmlTextError<A, B> {}
    impl<E: FromHtmlError> FromHtmlError for ListElementError<E> {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::never::Never;

    impl FromHtmlError for String {}

    fn ok<T>(s: &str) -> Result<String, T> {
        Ok(s.to_string())
    }

    fn err<T>(s: &str) -> Result<T, String> {
        Err(s.to_string())
    }

    #[test]
    fn vec() {
        assert_eq!(
            Vec::<FromHtmlImpl>::from_html::<MockElement>(
                &vec![MockElement(ok("a")), MockElement(ok("b"))],
                &()
            ),
            Ok(vec![FromHtmlImpl::new("a"), FromHtmlImpl::new("b")]),
            "the method is applied for each items of the vec"
        );

        assert_eq!(
            Vec::<FromHtmlImpl>::from_html::<MockElement>(
                &vec![MockElement(Ok("a".into())), MockElement(err("err!"))],
                &()
            ),
            Err(ListElementError {
                index: 1,
                error: "err!".to_string()
            }),
            "returned error if one of the vec items fails to apply"
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            Option::<FromHtmlImpl>::from_html::<MockElement>(&Some(MockElement(ok("ok!"))), &()),
            Ok(Some(FromHtmlImpl::new("ok!"))),
            "the method is applied for is present"
        );

        assert_eq!(
            Option::<FromHtmlImpl>::from_html::<MockElement>(&None, &()),
            Ok(None),
            "returned none if none"
        );

        assert_eq!(
            Option::<FromHtmlImpl>::from_html::<MockElement>(&Some(MockElement(err("err!"))), &()),
            err("err!"),
            "returned error if failed to apply"
        );
    }

    #[test]
    fn from_text() {
        struct MockExtractor {}
        impl TextExtractor for MockExtractor {
            type Error = Never;

            fn extract<N: HtmlNode>(&self, _source: &N) -> Result<String, Self::Error> {
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

    impl FromHtml<()> for FromHtmlImpl {
        type Source<N: HtmlNode> = MockElement;
        type Error = String;

        fn from_html<N: HtmlNode>(source: &MockElement, _args: &()) -> Result<Self, Self::Error> {
            source.clone().0.map(FromHtmlImpl)
        }
    }

    #[derive(Clone)]
    pub struct MockElement(Result<String, String>);
    pub struct MockSelector;

    impl Selector for MockSelector {
        type Error = Never;

        fn parse<S: AsRef<str>>(_s: S) -> Result<Self, Self::Error> {
            todo!()
        }
    }

    impl HtmlNode for MockElement {
        type Selector = MockSelector;

        fn select(&self, _sel: &Self::Selector) -> Vec<Self> {
            todo!()
        }

        fn text_contents(&self) -> String {
            todo!()
        }

        fn get_attribute<S: AsRef<str>>(&self, _attr: S) -> Option<&str> {
            todo!()
        }
    }
}
