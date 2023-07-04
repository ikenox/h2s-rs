use crate::{Error, FromHtml};

/// `Mapper` maps `F<N: HtmlNode>` -> `Result<F<T: FromHtml>>`
pub trait Mapper<T>: Sized {
    type Structure<U>;
    type Error<E: Error>: Error;

    fn try_map<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
    where
        F: Fn(A) -> Result<T, E>,
        E: Error;
}

pub mod impls {
    use super::*;

    impl<T> Mapper<T> for T
    where
        T: FromHtml,
    {
        type Structure<U> = U;
        type Error<E: Error> = E;

        fn try_map<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
        where
            F: Fn(A) -> Result<T, E>,
            E: Error,
        {
            f(source)
        }
    }

    impl<T> Mapper<T> for Option<T> {
        type Structure<U> = Option<U>;
        type Error<E: Error> = E;

        fn try_map<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
        where
            F: Fn(A) -> Result<T, E>,
            E: Error,
        {
            source.map(f).map_or(Ok(None), |v| v.map(Some))
        }
    }

    impl<T> Mapper<T> for Vec<T> {
        type Structure<U> = Vec<U>;
        type Error<E: Error> = ListElementError<E>;

        fn try_map<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
        where
            F: Fn(A) -> Result<T, E>,
            E: Error,
        {
            source
                .into_iter()
                .enumerate()
                .map(|(i, n)| f(n).map_err(|e| ListElementError { index: i, error: e }))
                // unwrapping results
                .try_fold(vec![], |mut acc, res| {
                    acc.push(res?);
                    Ok(acc)
                })
        }
    }

    impl<T, const M: usize> Mapper<T> for [T; M] {
        type Structure<U> = [U; M];
        type Error<E: Error> = ListElementError<E>;

        fn try_map<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
        where
            F: Fn(A) -> Result<T, E>,
            E: Error,
        {
            let v = source
                .into_iter()
                .enumerate()
                .map(|(i, n)| f(n).map_err(|e| ListElementError { index: i, error: e }))
                .try_fold(vec![], |mut acc, res| {
                    acc.push(res?);
                    Ok(acc)
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
    use super::*;
    use crate::mapper::impls::ListElementError;
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ErrorImpl(&'static str);

    impl Display for ErrorImpl {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", &self.0)
        }
    }
    impl std::error::Error for ErrorImpl {}

    #[test]
    fn vec() {
        assert_eq!(
            Mapper::try_map(vec!["a", "b"], try_map_func),
            Ok(vec!["a", "b"]),
            "the method is applied for each items of the vec"
        );

        assert_eq!(
            Mapper::try_map(vec!["a", "!b", "!c"], try_map_func) as Result<Vec<_>, _>,
            Err(ListElementError {
                index: 1,
                error: ErrorImpl("!b")
            }),
            "returned error if one of the vec items fails to apply"
        );
    }

    #[test]
    fn array() {
        assert_eq!(
            Mapper::try_map(["a", "b"], try_map_func),
            Ok(["a", "b"]),
            "the method is applied for each items of the array"
        );

        assert_eq!(
            Mapper::try_map(["a", "!b", "!c"], try_map_func) as Result<[_; 3], _>,
            Err(ListElementError {
                index: 1,
                error: ErrorImpl("!b")
            }),
            "returned error if one of the array items fails to apply"
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            Mapper::try_map(Some("ok"), try_map_func),
            Ok(Some("ok")),
            "the method is applied for is present"
        );

        assert_eq!(
            Mapper::try_map(None, try_map_func),
            Ok(None),
            "returned none if none"
        );

        assert_eq!(
            Mapper::try_map(Some("!err"), try_map_func) as Result<Option<_>, _>,
            Err(ErrorImpl("!err")),
            "returned error if failed to apply"
        );
    }

    fn try_map_func(s: &'static str) -> Result<&str, ErrorImpl> {
        if s.starts_with('!') {
            Err(ErrorImpl(s))
        } else {
            Ok(s)
        }
    }
}
