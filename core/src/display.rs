//! All implementations of Display trait.
//! Defining human-readable string is a different context from HTML-parsing process, so separate it and aggregate implementations here

use std::fmt::{Display, Formatter};

use crate::element_selector::{Root, Select};
use crate::extraction_method::{
    AttributeNotFound, ExtractAttribute, ExtractInnerText, ExtractionMethod, NoOp,
};
use crate::functor::ExactlyOne;
use crate::macro_utils::{ExtractionError, ParseError, ProcessError, TransformError};
use crate::transformable::{VecToArrayError, VecToOptionError, VecToSingleError};
use crate::traversable_with_context::{Context, ListIndex, NoContext};
use crate::Never;
use crate::{Error, FieldError};

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

impl Display for Never {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // never reached here
        write!(f, "")
    }
}

impl Display for AttributeNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "an attribute `{}` not found in the target element",
            self.name
        )
    }
}

impl Display for FieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field_name, self.error)
    }
}

impl<A, B, C> Display for ProcessError<A, B, C>
where
    A: Error,
    B: Error,
    C: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TransformError(e) => write!(f, "{}", e),
            Self::ExtractionError(e) => write!(f, "{}", e),
            Self::ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl<E> Display for TransformError<Select, E>
where
    E: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mismatched number of selected elements by \"{}\": {}",
            self.selector, self.error
        )
    }
}

impl<E> Display for TransformError<Root, E>
where
    E: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // In fact, this error may never occur because transforming from root element always success
        // So the user may never see the error message
        write!(f, "mismatched structure: {}", self.error)
    }
}

impl<C, M> Display for ExtractionError<C, M>
where
    C: Context,
    M: ExtractionMethod,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: failed to extract value of {}: {}",
            self.context, self.extraction_method, self.error
        )
    }
}

impl<C, E> Display for ParseError<C, E>
where
    C: Context,
    E: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Just displaying an inner error because error is originally caused at more inside of the inner struct
        write!(f, "{}: {}", self.context, self.error)
    }
}

impl Display for Select {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.selector)
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-")
    }
}

impl Display for ExtractAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "attribute={}", self.name)
    }
}
impl Display for ExtractInnerText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "inner text")
    }
}
impl Display for NoOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "no-op")
    }
}

impl<T> Display for ExactlyOne<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExactlyOne({})", self.0)
    }
}

impl Display for ListIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

impl Display for NoContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
