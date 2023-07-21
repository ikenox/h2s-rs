//! Implementations of `std::error::Error`

use std::fmt::{Debug, Display};

use crate::element_selector::TargetElementSelector;
use crate::extraction_method::{AttributeNotFound, ExtractionMethod};
use crate::functor::ExactlyOne;
use crate::macro_utils::{ExtractionError, ParseError, ProcessError, TransformError};
use crate::transformable::{VecToArrayError, VecToOptionError, VecToSingleError};
use crate::traversable_with_context::Context;
use crate::{Error, FieldError, Never};

impl Error for VecToArrayError {}
impl Error for VecToSingleError {}
impl Error for VecToOptionError {}
impl Error for AttributeNotFound {}
impl<E> Error for ExactlyOne<E> where E: Error {}
impl Error for Never {}
impl Error for FieldError {}

impl<S, E> Error for TransformError<S, E>
where
    Self: Display,
    S: TargetElementSelector,
    E: Error,
{
}

impl<C, M> Error for ExtractionError<C, M>
where
    C: Context,
    M: ExtractionMethod,
{
}

impl<C, E> Error for ParseError<C, E>
where
    C: Context,
    E: Error,
{
}

impl<A, B, C> Error for ProcessError<A, B, C> where Self: Display + Debug {}
