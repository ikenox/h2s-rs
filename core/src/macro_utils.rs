use crate::{ExtractionArgs, ExtractionError, ExtractionSource, Foo, FromHtml, HtmlNode, IntoArgs};

pub fn build_struct_field_value<
    T: FromHtml<Args = A, Source = S>,
    A: ExtractionArgs + 'static,
    S: ExtractionSource<Es = N> + 'static,
    B: IntoArgs<A>,
    N: HtmlNode,
>(
    node: &N,
    selector: Option<&'static str>,
    args_builder: &B,
) -> Result<T, ExtractionError> {
    let source = match &selector {
        Some(sel) => {
            S::build_source_from(Foo::List(
                node
                    // This won't fail because we should check the selector validity at compile time
                    .selected(sel)
                    .map_err(|_| {
                        ExtractionError::Unexpected(format!("invalid css selector: `{}`", sel))
                    })?,
            ))
        }
        None => S::build_source_from(Foo::Single(node.clone())),
    }
    .map_err(|e| ExtractionError::HtmlStructureUnmatched(e))?;
    T::extract_from(&source, &args_builder.build_args()).map_err(|inner| ExtractionError::Child {
        selector: selector.as_ref().map(|s| s.to_string()),
        args: Box::new(args_builder.build_args()),
        error: Box::new(inner),
    })
}
