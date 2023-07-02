use crate::functional::Identity;
use crate::Error;
use crate::Never;

/// `Transformer` tries to transform `F<T>` -> `G<T>`.
pub trait Transformable<T> {
    type Error: Error;
    fn try_transform(self) -> Result<T, Self::Error>;
}

impl<N> Transformable<Identity<N>> for Vec<N> {
    type Error = VecToSingleError;

    fn try_transform(mut self) -> Result<Identity<N>, Self::Error> {
        if self.len() > 1 {
            Err(VecToSingleError::TooManyElements { found: self.len() })
        } else {
            self.pop().map(Identity).ok_or(VecToSingleError::NoElements)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecToSingleError {
    TooManyElements { found: usize },
    NoElements,
}

impl<N, const A: usize> Transformable<[N; A]> for Vec<N> {
    type Error = VecToArrayError;

    fn try_transform(self) -> Result<[N; A], Self::Error> {
        self.try_into()
            .map_err(|v: Vec<_>| Self::Error::ElementNumberUnmatched {
                expected: A,
                found: v.len(),
            })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecToArrayError {
    ElementNumberUnmatched { expected: usize, found: usize },
}

impl<T> Transformable<T> for T {
    type Error = Never;

    fn try_transform(self) -> Result<T, Self::Error> {
        Ok(self)
    }
}

impl<T> Transformable<Identity<T>> for T {
    type Error = Never;

    fn try_transform(self) -> Result<Identity<T>, Self::Error> {
        Ok(Identity(self))
    }
}

impl<N> Transformable<Option<N>> for Vec<N> {
    type Error = VecToOptionError;

    fn try_transform(mut self) -> Result<Option<N>, Self::Error> {
        if self.len() > 1 {
            Err(Self::Error::TooManyElements { found: self.len() })
        } else {
            Ok(self.pop())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecToOptionError {
    TooManyElements { found: usize },
}

#[cfg(test)]
mod test {
    use crate::functional::Identity;
    use crate::transformer::Transformable;
    use crate::transformer::{VecToArrayError, VecToOptionError, VecToSingleError};

    #[test]
    fn vec_to_single() {
        assert_eq!(
            vec![0].try_transform() as Result<Identity<i32>, _>,
            Ok(Identity(0))
        );
        assert_eq!(
            vec![].try_transform() as Result<Identity<i32>, _>,
            Err(VecToSingleError::NoElements),
        );
        assert_eq!(
            vec![0, 1].try_transform() as Result<Identity<i32>, _>,
            Err(VecToSingleError::TooManyElements { found: 2 }),
        );
    }

    #[test]
    fn vec_to_array() {
        assert_eq!(vec!["foo", "bar"].try_transform(), Ok(["foo", "bar"]));
        assert_eq!(
            vec!["foo", "var"].try_transform() as Result<[&str; 3], _>,
            Err(VecToArrayError::ElementNumberUnmatched {
                expected: 3,
                found: 2
            }),
        );
    }

    #[test]
    fn vec_to_option() {
        assert_eq!(
            (vec![] as Vec<&str>).try_transform(),
            Ok(None) as Result<Option<&str>, _>
        );
        assert_eq!(vec!["foo"].try_transform(), Ok(Some("foo")));
        assert_eq!(
            vec!["foo", "var"].try_transform() as Result<Option<_>, _>,
            Err(VecToOptionError::TooManyElements { found: 2 })
        );
    }
}
