use crate::functor::ExactlyOne;
use crate::Error;
use crate::Never;

/// Tries to transform from `F<T>` to `G<T>`.
pub trait TransformableFrom<T>: Sized {
    type Error: Error;
    fn try_transform_from(t: T) -> Result<Self, Self::Error>;
}

impl<T> TransformableFrom<T> for T {
    type Error = Never;

    fn try_transform_from(t: T) -> Result<Self, Self::Error> {
        Ok(t)
    }
}

impl<T> TransformableFrom<Vec<T>> for ExactlyOne<T> {
    type Error = VecToSingleError;

    fn try_transform_from(mut t: Vec<T>) -> Result<Self, Self::Error> {
        if t.len() > 1 {
            Err(VecToSingleError::TooManyElements { found: t.len() })
        } else {
            t.pop().map(ExactlyOne).ok_or(VecToSingleError::NoElements)
        }
    }
}

impl<N, const A: usize> TransformableFrom<Vec<N>> for [N; A] {
    type Error = VecToArrayError;

    fn try_transform_from(t: Vec<N>) -> Result<Self, Self::Error> {
        t.try_into()
            .map_err(|v: Vec<_>| VecToArrayError::ElementNumberUnmatched {
                expected: A,
                found: v.len(),
            })
    }
}

impl<N> TransformableFrom<Vec<N>> for Option<N> {
    type Error = VecToOptionError;

    fn try_transform_from(mut t: Vec<N>) -> Result<Self, Self::Error> {
        if t.len() > 1 {
            Err(Self::Error::TooManyElements { found: t.len() })
        } else {
            Ok(t.pop())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecToSingleError {
    TooManyElements { found: usize },
    NoElements,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecToArrayError {
    ElementNumberUnmatched { expected: usize, found: usize },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecToOptionError {
    TooManyElements { found: usize },
}

#[cfg(test)]
mod test {
    use crate::functor::ExactlyOne;
    use crate::transformable::TransformableFrom;
    use crate::transformable::{VecToArrayError, VecToOptionError, VecToSingleError};

    #[test]
    fn identity() {
        assert_eq!(
            ExactlyOne::try_transform_from(ExactlyOne(0)),
            Ok(ExactlyOne(0))
        );
    }

    #[test]
    fn vec_to_single() {
        assert_eq!(ExactlyOne::try_transform_from(vec![0]), Ok(ExactlyOne(0)));
        assert_eq!(
            ExactlyOne::<()>::try_transform_from(vec![]),
            Err(VecToSingleError::NoElements),
        );
        assert_eq!(
            ExactlyOne::try_transform_from(vec![0, 1]),
            Err(VecToSingleError::TooManyElements { found: 2 }),
        );
    }

    #[test]
    fn vec_to_array() {
        assert_eq!(
            <[_; 2]>::try_transform_from(vec!["foo", "bar"]),
            Ok(["foo", "bar"])
        );
        assert_eq!(
            <[&str; 3]>::try_transform_from(vec!["foo", "var"]),
            Err(VecToArrayError::ElementNumberUnmatched {
                expected: 3,
                found: 2
            }),
        );
    }

    #[test]
    fn vec_to_option() {
        assert_eq!(Option::<()>::try_transform_from(vec![]), Ok(None),);
        assert_eq!(Option::try_transform_from(vec!["foo"]), Ok(Some("foo")));
        assert_eq!(
            Option::try_transform_from(vec!["foo", "var"]),
            Err(VecToOptionError::TooManyElements { found: 2 })
        );
    }
}
