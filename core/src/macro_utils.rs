//! A set of internal utility methods that will be used in the auto-generated code on `FromHtml` derive macro.
//! These methods are shorthands to reduce codes in the `quote!` macro. It improves development experience with IDE.
//! You wouldn't call these methods directly in your code.

use crate::from_html::{ExtractionType, StructErrorCause, StructFieldError};
use crate::mapper::{Functor, Mapper, Traversable};
use crate::transformer::{Transformer, VecToArrayError};
use crate::Error;
use crate::{CssSelector, FromHtml, HtmlNode};

pub fn select<N>(source: &N, selector: &'static str) -> Vec<N>
where
    N: HtmlNode,
{
    // TODO cache parsed selector
    let selector = N::Selector::parse(selector)
        // this should be never failed because the selector validity has been checked at compile-time
        // TODO avoid unwrap
        .unwrap();
    source.select(&selector)
}

pub fn try_transform_and_map2<N, T, F, S>(
    source: S,
    args: &T::Args,
    selector: Option<&'static str>,
    field_name: &'static str,
) -> Result<F, Box<dyn Error>>
where
    N: HtmlNode,
    T: FromHtml,
    F: Traversable<Inner = T>,
    S: Transformer<F::This1<N>>,
{
    let a: F::This1<N> = source.try_transform().unwrap();
    let b = a.fmap(|n| T::from_html(&n, args));
    Err(Box::new(VecToArrayError::ElementNumberUnmatched {
        expected: 0,
        found: 0,
    }))
}

pub fn try_transform_and_map<N, T, M, S>(
    source: S,
    args: &T::Args,
    selector: Option<&'static str>,
    field_name: &'static str,
) -> Result<M, Box<dyn Error>>
where
    N: HtmlNode,
    T: FromHtml,
    M: Mapper<T>,
    S: Transformer<M::Structure<N>>,
{
    source
        .try_transform()
        .map_err(StructErrorCause::StructureUnmatched)
        .and_then(|s| M::try_map(s, args).map_err(StructErrorCause::ParseError))
        .map_err(|e| StructFieldError {
            field_name: field_name.to_string(),
            selector: selector.map(|a| a.to_string()),
            error: e,
        })
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn default_argument<T>() -> T
where
    T: DefaultArg,
{
    DefaultArg::default()
}

pub trait DefaultArg {
    fn default() -> Self;
}

impl DefaultArg for () {
    fn default() -> Self {}
}

impl DefaultArg for ExtractionType {
    fn default() -> Self {
        ExtractionType::Text
    }
}
