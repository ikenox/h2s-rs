//! Implementations of `std::error::Error`

use crate::from_html::{FromHtmlTextError, StructFieldError};
use crate::mapper::impls::ListElementError;
use crate::text_extractor::impls::AttributeNotFound;
use crate::transformer::{VecToArrayError, VecToOptionError, VecToSingleError};
use crate::{Error, Never};

impl std::error::Error for VecToArrayError {}
impl std::error::Error for VecToSingleError {}
impl std::error::Error for VecToOptionError {}
impl std::error::Error for AttributeNotFound {}
impl<A> std::error::Error for ListElementError<A> where A: Error {}
impl<A, B> std::error::Error for FromHtmlTextError<A, B>
where
    A: Error,
    B: Error,
{
}
impl<A, B> std::error::Error for StructFieldError<A, B>
where
    A: Error,
    B: Error,
{
}
impl std::error::Error for Never {}
impl std::error::Error for Box<dyn Error> {}
