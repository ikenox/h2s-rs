use crate::FieldValue;
use crate::{Error, FromHtml};

use super::*;

impl<T> FieldValue for T
where
    T: FromHtml,
{
    type Inner = T;
    type Structure<U> = U;
    type Error<E: Error> = E;

    fn try_traverse_from<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
    where
        F: Fn(A) -> Result<Self::Inner, E>,
        E: Error,
    {
        f(source)
    }
}

impl<T> FieldValue for Option<T>
where
    T: FromHtml,
{
    type Inner = T;
    type Structure<U> = Option<U>;
    type Error<E: Error> = E;

    fn try_traverse_from<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
    where
        F: Fn(A) -> Result<Self::Inner, E>,
        E: Error,
    {
        source.map(f).map_or(Ok(None), |v| v.map(Some))
    }
}

impl<T> FieldValue for Vec<T>
where
    T: FromHtml,
{
    type Inner = T;
    type Structure<U> = Vec<U>;
    type Error<E: Error> = ListElementError<E>;

    fn try_traverse_from<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
    where
        F: Fn(A) -> Result<Self::Inner, E>,
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

impl<T, const M: usize> FieldValue for [T; M]
where
    T: FromHtml,
{
    type Inner = T;
    type Structure<U> = [U; M];
    type Error<E: Error> = ListElementError<E>;

    fn try_traverse_from<A, E, F>(source: Self::Structure<A>, f: F) -> Result<Self, Self::Error<E>>
    where
        F: Fn(A) -> Result<Self::Inner, E>,
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

#[cfg(test)]
mod test {
    use std::fmt::{Display, Formatter};

    use crate::field_value::ListElementError;
    use crate::{FieldValue, HtmlNode};

    use super::*;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ErrorImpl(&'static str);

    impl Display for ErrorImpl {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", &self.0)
        }
    }
    impl std::error::Error for ErrorImpl {}
    impl FromHtml for &str {
        type Args = ();
        type Error = ErrorImpl;

        fn from_html<N>(source: &N, args: &Self::Args) -> Result<Self, Self::Error>
        where
            N: HtmlNode,
        {
            todo!()
        }
    }

    #[test]
    fn vec() {
        assert_eq!(
            FieldValue::try_traverse_from(vec!["a", "b"], try_map_func),
            Ok(vec!["a", "b"]),
            "the method is applied for each items of the vec"
        );

        assert_eq!(
            FieldValue::try_traverse_from(vec!["a", "!b", "!c"], try_map_func) as Result<Vec<_>, _>,
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
            FieldValue::try_traverse_from(["a", "b"], try_map_func),
            Ok(["a", "b"]),
            "the method is applied for each items of the array"
        );

        assert_eq!(
            FieldValue::try_traverse_from(["a", "!b", "!c"], try_map_func) as Result<[_; 3], _>,
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
            FieldValue::try_traverse_from(Some("ok"), try_map_func),
            Ok(Some("ok")),
            "the method is applied for is present"
        );

        assert_eq!(
            FieldValue::try_traverse_from(None, try_map_func),
            Ok(None),
            "returned none if none"
        );

        assert_eq!(
            FieldValue::try_traverse_from(Some("!err"), try_map_func) as Result<Option<_>, _>,
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
