//! A core part of h2s

use html::HtmlElement;
use std::error::Error;
use std::fmt::Debug;

use parseable::Parseable;

use crate::functor::Functor;

pub mod display;
pub mod element_selector;
pub mod error;
pub mod extraction_method;
pub mod field_value;
pub mod functor;
pub mod html;
pub mod macro_utils;
pub mod parseable;
pub mod transformable;
pub mod traversable;
pub mod traversable_with_context;

/// A converter from single HTML element to single struct
pub trait FromHtml: Sized {
    type Error: Error;

    fn from_html<N>(input: N) -> Result<Self, Self::Error>
    where
        N: HtmlElement;
}

#[derive(Debug)]
pub struct FieldError {
    pub field_name: String,
    pub error: Box<dyn Error>,
}

/// Similar with std::convert::Infallible
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Never {}

// TODO remove?
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tuple<T, U>(pub T, pub U);
