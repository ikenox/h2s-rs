use crate::ExtractionError::HtmlStructureUnmatched;
use crate::{
    extract_from, ArgBuilder, ExtractionArgs, ExtractionError, FromHtml, HtmlElements, IntoArgs,
};
use kuchiki::{ElementData, NodeData, NodeDataRef, NodeRef};
use std::rc::Rc;

pub fn build_struct_field_value<
    'a,
    T: FromHtml<Args = A>,
    A: ExtractionArgs + 'static,
    B: IntoArgs<A>,
>(
    node: &NodeRef,
    selector: impl AsRef<str>,
    args_builder: &B,
) -> Result<T, ExtractionError> {
    let select = node
        // This won't fail because we should check the selector validity at compile time
        .select(selector.as_ref())
        .map_err(|_| {
            ExtractionError::Unexpected(format!("invalid css selector: `{}`", selector.as_ref()))
        })?;
    extract_from(select, &args_builder.build_args()).map_err(|inner| ExtractionError::Child {
        selector: selector.as_ref().to_string(),
        args: Rc::new(args_builder.build_args()),
        error: Box::new(inner),
    })
}

pub fn get_single_node_for_build_struct<E: HtmlElements>(
    e: E,
) -> Result<NodeDataRef<ElementData>, ExtractionError> {
    e.get_exactly_one().map_err(|e| HtmlStructureUnmatched(e))
}
