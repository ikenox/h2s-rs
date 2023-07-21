use crate::functor::ExactlyOne;
use crate::traversable_with_context::FunctorWithContext;

use super::*;

/// A field value of FromHtml-deriving struct
pub trait FieldValue: Sized {
    type Inner: Parseable;
    /// An intermediate representation of the field value
    type Wrapped: FunctorWithContext<Inner = Self::Inner>;

    fn finalize(wrapped: <Self::Wrapped as Functor>::Structure<Self::Inner>) -> Self;
}

impl<T> FieldValue for T
where
    T: Parseable,
{
    type Inner = T;
    type Wrapped = ExactlyOne<T>;

    fn finalize(wrapped: Self::Wrapped) -> Self {
        wrapped.0
    }
}

impl<T> FieldValue for Option<T>
where
    T: Parseable,
{
    type Inner = T;
    type Wrapped = Self;

    fn finalize(wrapped: Self::Wrapped) -> Self {
        wrapped
    }
}

impl<T> FieldValue for Vec<T>
where
    T: Parseable,
{
    type Inner = T;
    // TODO use iterator as intermediate data representation to avoid repeating into_iter() and collect()
    type Wrapped = Self;

    fn finalize(wrapped: Self::Wrapped) -> Self {
        wrapped
    }
}

impl<T, const M: usize> FieldValue for [T; M]
where
    T: Parseable,
{
    type Inner = T;
    type Wrapped = Self;

    fn finalize(wrapped: Self::Wrapped) -> Self {
        wrapped
    }
}
