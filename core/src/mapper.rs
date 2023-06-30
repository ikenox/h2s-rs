use crate::{Error, FromHtml, HtmlNode};

pub struct Identity<T>(pub T);

pub trait Functor {
    type Inner;
    type This<A>: Functor<Inner = A>;
    fn fmap<B>(self, f: impl Fn(Self::Inner) -> B) -> Self::This<B>;
}

pub trait Applicative: Functor {
    fn pure(inner: Self::Inner) -> Self;
    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B;
}

pub trait Monad: Applicative {
    fn bind<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> Self::This<B>;
}

impl<T> Functor for Identity<T> {
    type Inner = T;
    type This<A> = Identity<A>;

    fn fmap<B>(self, f: impl Fn(Self::Inner) -> B) -> Self::This<B> {
        Identity(f(self.0))
    }
}

impl <T> Applicative for Identity<T> {
    fn pure(inner: Self::Inner) -> Self {
        Identity(inner)
    }

    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B> where F: Fn(Self::Inner) -> B {
        todo!()
    }
}

impl <T> Monad for Identity<T> {
    fn bind<B, F>(self, f: F) -> Self::This<B> where F: Fn(Self::Inner) -> Self::This<B> {
        f(self.0)
    }
}

impl<T> Functor for Option<T> {
    type Inner = T;
    type This<A> = Option<A>;

    fn fmap<B>(self, f: impl Fn(Self::Inner) -> B) -> Self::This<B> {
        self.map(f)
    }
}

impl<T, E> Functor for Result<T, E> {
    type Inner = T;
    type This<A> = Result<A, E>;

    fn fmap<B>(self, f: impl Fn(Self::Inner) -> B) -> Self::This<B> {
        self.bind(|a| <Self::This<B>>::pure(f(a)))
    }
}

impl<T, E> Applicative for Result<T, E> {
    fn pure(inner: Self::Inner) -> Self {
        Ok(inner)
    }

    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        // let func = f.bind(|f| self.bind(|a| f(a)));
        //
        self.bind(|a| f.bind(|f| <Self::This<B>>::pure(f(a))))
    }
}

impl<T, E> Monad for Result<T, E> {
    fn bind<B, F>(self, f: F) -> Self::This<B>
    where
        F: FnOnce(Self::Inner) -> Self::This<B>,
    {
        self.and_then(|a| f(a))
    }
}

impl<T> Functor for Vec<T> {
    type Inner = T;
    type This<A> = Vec<A>;

    fn fmap<B>(self, f: impl Fn(Self::Inner) -> B) -> Self::This<B> {
        todo!()
    }
}

impl<T> Applicative for Vec<T> {
    fn pure(inner: Self::Inner) -> Self {
        vec![inner]
    }

    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        let hoge = f.bind(|f| <Self::This<F>>::pure(f));
        let ff=|a:T|;
        self.bind(|elem|ff(elem))
    }
}

impl<T> Monad for Vec<T> {
    fn bind<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> Self::This<B>,
    {
        self.into_iter().flat_map(f).collect()
    }
}

pub trait Foldable {}

pub trait Traversable: Functor + Foldable {
    fn traverse<A, F>(self, f: F) -> A::This<Self::This<A::Inner>>
    where
        A: Applicative,
        F: Fn(Self::Inner) -> A;
}

/// `Mapper` maps `F<N: HtmlNode>` -> `Result<F<T: FromHtml>>`
pub trait Mapper<T>: Sized {
    type Structure<U>;
    type Error<E: Error>: Error;

    // TODO separate process of folding the error
    fn try_map<N: HtmlNode>(
        source: Self::Structure<N>,
        args: &T::Args,
    ) -> Result<Self, Self::Error<T::Error>>
    where
        T: FromHtml;
}

pub mod impls {
    use super::*;

    impl<T> Mapper<T> for T
    where
        T: FromHtml,
    {
        type Structure<U> = U;
        type Error<E: Error> = E;

        fn try_map<N: HtmlNode>(
            source: Self::Structure<N>,
            args: &T::Args,
        ) -> Result<Self, Self::Error<T::Error>>
        where
            T: FromHtml,
        {
            T::from_html(&source, args)
        }
    }

    impl<T> Mapper<T> for Option<T> {
        type Structure<U> = Option<U>;
        type Error<E: Error> = E;

        fn try_map<N>(
            source: Self::Structure<N>,
            args: &T::Args,
        ) -> Result<Self, Self::Error<T::Error>>
        where
            T: FromHtml,
            N: HtmlNode,
        {
            source
                .as_ref()
                .map(|n| T::from_html(n, args))
                .map_or(Ok(None), |v| v.map(Some))
        }
    }

    impl<T> Mapper<T> for Vec<T> {
        type Structure<U> = Vec<U>;
        type Error<E: Error> = ListElementError<E>;

        fn try_map<N>(
            source: Self::Structure<N>,
            args: &T::Args,
        ) -> Result<Self, Self::Error<T::Error>>
        where
            T: FromHtml,
            N: HtmlNode,
        {
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

    impl<T, const M: usize> Mapper<T> for [T; M] {
        type Structure<U> = [U; M];
        type Error<E: Error> = ListElementError<E>;

        fn try_map<N>(
            source: Self::Structure<N>,
            args: &T::Args,
        ) -> Result<Self, Self::Error<T::Error>>
        where
            T: FromHtml,
            N: HtmlNode,
        {
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

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ListElementError<E: Error> {
        pub index: usize,
        pub error: E,
    }
}

#[cfg(test)]
mod test {
    use std::fmt::{Display, Formatter};

    use crate::mapper::impls::ListElementError;
    use crate::CssSelector;
    use crate::Never;

    use super::*;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ErrorImpl(String);

    impl Display for ErrorImpl {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", &self.0)
        }
    }
    impl std::error::Error for ErrorImpl {}

    #[test]
    fn vec() {
        assert_eq!(
            Vec::<FromHtmlImpl>::try_map(vec![MockElement("a"), MockElement("b")], &()),
            Ok(vec![FromHtmlImpl::new("a"), FromHtmlImpl::new("b")]),
            "the method is applied for each items of the vec"
        );

        assert_eq!(
            Vec::<FromHtmlImpl>::try_map(vec![MockElement("a"), MockElement("!b")], &()),
            Err(ListElementError {
                index: 1,
                error: ErrorImpl("!b".to_string())
            }),
            "returned error if one of the vec items fails to apply"
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            Option::<FromHtmlImpl>::try_map::<MockElement>(Some(MockElement("ok!")), &()),
            Ok(Some(FromHtmlImpl::new("ok!"))),
            "the method is applied for is present"
        );

        assert_eq!(
            Option::<FromHtmlImpl>::try_map::<MockElement>(None, &()),
            Ok(None),
            "returned none if none"
        );

        assert_eq!(
            Option::<FromHtmlImpl>::try_map::<MockElement>(Some(MockElement("!err")), &()),
            Err(ErrorImpl("!err".to_string())),
            "returned error if failed to apply"
        );
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct FromHtmlImpl(String);

    impl FromHtmlImpl {
        pub fn new<S: AsRef<str>>(s: S) -> Self {
            Self(s.as_ref().to_string())
        }
    }

    impl FromHtml for FromHtmlImpl {
        type Args = ();
        type Error = ErrorImpl;

        fn from_html<N>(source: &N, _args: &Self::Args) -> Result<Self, Self::Error>
        where
            N: HtmlNode,
        {
            let text = source.text_contents();
            if text.starts_with('!') {
                Err(ErrorImpl(text))
            } else {
                Ok(FromHtmlImpl(text))
            }
        }
    }

    #[derive(Clone)]
    pub struct MockElement(&'static str);
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

        fn select(&self, _selector: &Self::Selector) -> Vec<Self> {
            unimplemented!()
        }

        fn text_contents(&self) -> String {
            self.0.to_string()
        }

        fn attribute<S>(&self, _attr: S) -> Option<&str>
        where
            S: AsRef<str>,
        {
            unimplemented!()
        }
    }
}
