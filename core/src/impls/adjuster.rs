use crate::never::Never;
use crate::*;
use std::fmt::Formatter;

pub trait StructureAdjuster<N> {
    type Error: AdjustStructureError;
    fn try_adjust(self) -> Result<N, Self::Error>;
}

pub trait AdjustStructureError: Display + Debug + 'static {}

impl<N> StructureAdjuster<N> for N {
    type Error = Never;

    fn try_adjust(self) -> Result<N, Self::Error> {
        Ok(self)
    }
}

impl<N> StructureAdjuster<N> for Vec<N> {
    type Error = VecToSingleError;

    fn try_adjust(mut self) -> Result<N, Self::Error> {
        if self.len() > 1 {
            Err(Self::Error::TooManyElements { found: self.len() })
        } else {
            self.pop().ok_or_else(|| Self::Error::NoElements)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecToSingleError {
    TooManyElements { found: usize },
    NoElements,
}

impl<N, const A: usize> StructureAdjuster<[N; A]> for Vec<N> {
    type Error = VecToArrayError;

    fn try_adjust(self) -> Result<[N; A], Self::Error> {
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

impl<N> StructureAdjuster<Option<N>> for Vec<N> {
    type Error = VecToOptionError;

    fn try_adjust(mut self) -> Result<Option<N>, Self::Error> {
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

mod display {
    use super::*;
    use std::fmt::{Display, Formatter};

    impl Display for VecToSingleError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self {
                VecToSingleError::TooManyElements { found } => {
                    write!(
                        f,
                        "expected exactly one element, but {found} elements found"
                    )
                }
                VecToSingleError::NoElements => {
                    write!(f, "expected exactly one element, but no elements found")
                }
            }
        }
    }

    impl Display for VecToOptionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self {
                VecToOptionError::TooManyElements { found } => {
                    write!(f, "expected 0 or 1 element, but found {found} elements")
                }
            }
        }
    }

    impl Display for VecToArrayError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self {
                VecToArrayError::ElementNumberUnmatched { expected, found } => {
                    write!(
                        f,
                        "expected {expected} elements, but found {found} elements"
                    )
                }
            }
        }
    }
}

mod error {
    use super::*;

    impl AdjustStructureError for VecToSingleError {}
    impl AdjustStructureError for VecToOptionError {}
    impl AdjustStructureError for VecToArrayError {}
}

#[cfg(test)]
mod test {
    use crate::impls::adjuster::StructureAdjuster;
    use crate::impls::adjuster::{VecToArrayError, VecToOptionError, VecToSingleError};

    #[test]
    fn single() {
        assert_eq!("foo".try_adjust(), Ok("foo"));
    }

    #[test]
    fn vec_to_single() {
        assert_eq!(vec!["foo"].try_adjust() as Result<&str, _>, Ok("foo"));
        assert_eq!(
            vec![].try_adjust() as Result<&str, _>,
            Err(VecToSingleError::NoElements),
        );
        assert_eq!(
            vec!["foo", "bar"].try_adjust() as Result<&str, _>,
            Err(VecToSingleError::TooManyElements { found: 2 }),
        );
    }

    #[test]
    fn vec_to_array() {
        assert_eq!(vec!["foo", "bar"].try_adjust(), Ok(["foo", "bar"]));
        assert_eq!(
            vec!["foo", "var"].try_adjust() as Result<[&str; 3], _>,
            Err(VecToArrayError::ElementNumberUnmatched {
                expected: 3,
                found: 2
            }),
        );
    }

    #[test]
    fn vec_to_option() {
        assert_eq!(
            (vec![] as Vec<&str>).try_adjust(),
            Ok(None) as Result<Option<&str>, _>
        );
        assert_eq!(vec!["foo"].try_adjust(), Ok(Some("foo")));
        assert_eq!(
            vec!["foo", "var"].try_adjust() as Result<Option<_>, _>,
            Err(VecToOptionError::TooManyElements { found: 2 })
        );
    }
}
