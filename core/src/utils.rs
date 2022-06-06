use crate::{ExtractionError, FromHtml};
use scraper::ElementRef;

#[cfg(feature = "backend-scraper")]
pub fn parse<H>(html: &str) -> Result<H, ExtractionError>
where
    for<'b, 'a> H: FromHtml<'b, (), Source<ElementRef<'a>> = ElementRef<'a>>,
{
    let doc = ::scraper::Html::parse_document(html);
    H::from_html::<ElementRef<'_>>(&doc.root_element(), ())
}
