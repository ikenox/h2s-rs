use crate::{extract, ExtractionError, Extractor, FromHtml, Nodes, StructExtractor, TendrilSink};

pub fn extract_from_html<H: FromHtml>(s: &str) -> Result<H, ExtractionError> {
    let doc = kuchiki::parse_html().one(s);
    H::from_html(&doc)
}
