use crate::{ExtractionError, FromHtml, StructExtractionArgs, TendrilSink};
use kuchiki::NodeRef;

pub fn extract_from_html<T: FromHtml<Source<NodeRef> = NodeRef, Args = StructExtractionArgs>>(
    s: &str,
) -> Result<T, ExtractionError> {
    let doc = kuchiki::parse_html().one(s);
    T::extract_from::<NodeRef>(&doc, &StructExtractionArgs)
}
