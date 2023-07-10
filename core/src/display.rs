//! All implementations of Display trait.
//! Defining human-readable string is a different context from HTML-parsing process, so we are separating it and aggregating implementations here

use std::fmt::{Display, Formatter};

use crate::field_value::ListElementError;
use crate::from_html::{FromHtmlTextError, StructErrorCause, StructFieldError};
use crate::text_extractor::impls::AttributeNotFound;
use crate::transformer::{VecToArrayError, VecToOptionError, VecToSingleError};
use crate::Error;
use crate::Never;

impl<A, B> Display for StructFieldError<A, B>
where
    A: Error,
    B: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: ",
            self.field_name,
            self.selector
                .as_ref()
                .map(|s| format!("({s})"))
                .unwrap_or_else(|| "".into()),
        )?;
        match &self.error {
            StructErrorCause::StructureUnmatched(e) => write!(f, "{e}"),
            StructErrorCause::ParseError(e) => write!(f, "{e}"),
        }
    }
}

impl<A, B> Display for FromHtmlTextError<A, B>
where
    A: Error,
    B: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to convert text into the value: ")?;
        match &self {
            FromHtmlTextError::ExtractionFailed(e) => {
                write!(f, "{e}")
            }
            FromHtmlTextError::TextParseError(e) => {
                write!(f, "{e}")
            }
        }
    }
}

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

impl<E> Display for ListElementError<E>
where
    E: Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.index, self.error)
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
