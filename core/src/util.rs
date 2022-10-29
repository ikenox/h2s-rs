//! A set of public utility methods that are convenient for users of this library.
//! This module doesn't represent any business logic. It's just a human-friendly user interface.

use crate::FromHtml;

pub fn parse<T>(html: &str) -> Result<T, T::Error>
where
    for<'b> T: FromHtml<Args = ()>,
{
    parse_with_args(html, &())
}

#[cfg(feature = "backend-scraper")]
pub fn parse_with_args<T: FromHtml>(html: &str, args: &T::Args) -> Result<T, T::Error> {
    let doc = scraper::Html::parse_document(html);
    T::from_html(&doc.root_element(), args)
}
