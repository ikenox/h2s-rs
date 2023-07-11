use crate::functor::{ExactlyOne, Functor};
use crate::Tuple;

/// Converts from `Structure<A>` and `fn(A) -> Result<T>` to `Result<Structure<T>>`
/// It works similar to `Traversable` in functional programming languages, but only for `Result` type
pub trait Traversable: Functor {
    fn traverse<A, B, E, F>(a: Self::Structure<A>, f: F) -> Result<Self::Structure<B>, E>
    where
        F: Fn(A) -> Result<B, E>;
}

impl<T> Traversable for ExactlyOne<T> {
    fn traverse<A, B, E, F>(a: Self::Structure<A>, f: F) -> Result<Self::Structure<B>, E>
    where
        F: Fn(A) -> Result<B, E>,
    {
        f(a.0).map(ExactlyOne)
    }
}

impl<T> Traversable for Option<T> {
    fn traverse<A, B, E, F>(a: Self::Structure<A>, f: F) -> Result<Self::Structure<B>, E>
    where
        F: Fn(A) -> Result<B, E>,
    {
        a.map(f).map_or(Ok(None), |v| v.map(Some))
    }
}

impl<T> Traversable for Vec<T> {
    fn traverse<A, B, E, F>(a: Self::Structure<A>, f: F) -> Result<Self::Structure<B>, E>
    where
        F: Fn(A) -> Result<B, E>,
    {
        a.into_iter().map(f).try_fold(vec![], |mut acc, res| {
            acc.push(res?);
            Ok(acc)
        })
    }
}

impl<T, const M: usize> Traversable for [T; M] {
    fn traverse<A, B, E, F>(a: Self::Structure<A>, f: F) -> Result<Self::Structure<B>, E>
    where
        F: Fn(A) -> Result<B, E>,
    {
        // TODO Replace with try_map when it is stabilized. Currently try_map is unstable future
        let v = a.into_iter().map(f).try_fold(vec![], |mut acc, res| {
            acc.push(res?);
            Ok(acc)
        })?;

        // this conversion should never fail
        // TODO avoid unwrap
        Ok(v.try_into().map_err(|_| "").unwrap())
    }
}

impl<T, U> Traversable for Tuple<T, U> {
    fn traverse<A, B, E, F>(a: Self::Structure<A>, f: F) -> Result<Self::Structure<B>, E>
    where
        F: Fn(A) -> Result<B, E>,
    {
        f(a.1).map(|b| Tuple(a.0, b))
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    use super::*;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ErrorImpl(&'static str);

    impl Error for ErrorImpl {}
    impl Display for ErrorImpl {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", &self.0)
        }
    }

    #[test]
    fn exactly_one() {
        assert_eq!(
            ExactlyOne::<()>::traverse(ExactlyOne("a"), try_func),
            Ok(ExactlyOne(1)),
            "a map function is just applied"
        );

        assert_eq!(
            ExactlyOne::<()>::traverse(ExactlyOne("!a"), try_func),
            Err(ErrorImpl("!a")),
            "returned error if a map function fails"
        );
    }

    #[test]
    fn vec() {
        assert_eq!(
            Vec::<()>::traverse(vec!["a", "bb"], try_func),
            Ok(vec![1, 2]),
            "a map function is applied for each items of the vec"
        );

        assert_eq!(
            Vec::<()>::traverse(vec!["a", "!b", "!c"], try_func),
            Err(ErrorImpl("!b")),
            "returned error if one of the vec items fails to apply"
        );
    }

    #[test]
    fn array() {
        assert_eq!(
            <[(); 2]>::traverse(["a", "bb"], try_func),
            Ok([1, 2]),
            "a map function is applied for each items of the array"
        );

        assert_eq!(
            <[(); 3]>::traverse(["a", "!b", "!c"], try_func),
            Err(ErrorImpl("!b")),
            "returned error if one of the array items fails to apply"
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            Option::<()>::traverse(Some("a"), try_func),
            Ok(Some(1)),
            "a map function is applied for is present"
        );

        assert_eq!(
            Option::<()>::traverse(None, try_func),
            Ok(None),
            "returned none if none"
        );

        assert_eq!(
            Option::<()>::traverse(Some("!err"), try_func),
            Err(ErrorImpl("!err")),
            "returned error if failed to apply"
        );
    }

    fn try_func(s: &'static str) -> Result<usize, ErrorImpl> {
        if s.starts_with('!') {
            Err(ErrorImpl(s))
        } else {
            Ok(s.len())
        }
    }
}
