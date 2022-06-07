//! A set of public utility methods that are convenient for users of this library.
//! This module doesn't represent any business logic. It's just a human-friendly user interface.

use crate::{FromHtml, ParseError};
use scraper::ElementRef;

#[cfg(feature = "backend-scraper")]
pub fn parse<T>(html: &str) -> Result<T, ParseError>
where
    for<'a, 'b> T: FromHtml<'a, (), Source<ElementRef<'b>> = ElementRef<'b>>,
{
    let doc = ::scraper::Html::parse_document(html);
    T::from_html::<ElementRef<'_>>(&doc.root_element(), ())
}

#[cfg(feature = "backend-scraper")]
pub fn parse_with_args<'a, T, A: 'a>(html: &str, args: A) -> Result<T, ParseError>
where
    for<'b> T: FromHtml<'a, A, Source<ElementRef<'b>> = ElementRef<'b>>,
{
    let doc = ::scraper::Html::parse_document(html);
    T::from_html::<ElementRef<'_>>(&doc.root_element(), args)
}