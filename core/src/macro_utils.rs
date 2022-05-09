use crate::ExtractionError::HtmlStructureUnmatched;
use crate::{extract_from, ArgBuilder, ExtractionError, FromHtml, HtmlElements, IntoArgs};
use kuchiki::{ElementData, NodeData, NodeDataRef, NodeRef};

pub fn build_struct_field_value<'a, T: FromHtml<Args = A>, A, B: IntoArgs<A>>(
    node: &NodeRef,
    // manipulation
    selector: impl AsRef<str>,
    args_builder: &B,
) -> Result<T, ExtractionError> {
    let select = node
        // This won't fail because we should check the selector validity at compile time
        .select(selector.as_ref())
        .map_err(|_| {
            ExtractionError::Unexpected(format!("invalid css selector: `{}`", selector.as_ref()))
        })?;
    extract_from(select, &args_builder.build_args())
}

pub fn get_single_node_for_build_struct<E: HtmlElements>(
    // target elements
    e: E,
) -> Result<NodeDataRef<ElementData>, ExtractionError> {
    e.get_exactly_one().map_err(|e| HtmlStructureUnmatched(e))
}
