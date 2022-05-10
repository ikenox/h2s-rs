use crate::{
    ExtractionError, FromHtml, GetElementError, HtmlNode, SelectError, TendrilSink,
    TextExtractionMethod,
};
use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::{ElementData, Node, NodeDataRef, NodeRef};
use std::borrow::Borrow;

impl FromHtml for String {
    type Source = NodeRef;
    type Args = TextExtractionMethod;

    fn extract_from(mut source: &Self::Source, args: &Self::Args) -> Result<Self, ExtractionError> {
        Ok(match args {
            TextExtractionMethod::TextContent => source.text_contents(),
            TextExtractionMethod::Attribute(attr) => source
                .as_element()
                .and_then(|e| {
                    e.borrow()
                        .attributes
                        .borrow()
                        .get(attr.as_str())
                        .map(|a| a.to_string())
                })
                .ok_or_else(|| ExtractionError::AttributeNotFound)?,
        })
    }
}

impl<T: FromHtml> FromHtml for Option<T> {
    type Source = Option<T::Source>;
    type Args = T::Args;

    fn extract_from(source: &Self::Source, args: &Self::Args) -> Result<Self, ExtractionError> {
        Ok(if let Some(source) = source {
            Some(
                T::extract_from(source, &args).map_err(|e| ExtractionError::Child {
                    selector: None, // todo?
                    // todo
                    // args: Box::new(args.clone()),
                    args: Box::new(()),
                    error: Box::new(e),
                })?,
            )
        } else {
            None
        })
    }
}

impl<T: FromHtml> FromHtml for Vec<T> {
    type Source = Vec<T::Source>;
    type Args = T::Args;
    fn extract_from(source: &Self::Source, args: &Self::Args) -> Result<Self, ExtractionError> {
        source
            .into_iter()
            .enumerate()
            .fold(Ok(vec![]), |acc, (i, source)| {
                acc.and_then(|v| T::extract_from(source, &args).map(|t| (v, t)))
                    .map(|(mut v, t)| {
                        v.push(t);
                        v
                    })
                    .map_err(|e| ExtractionError::Child {
                        selector: Some(format!("[{}]", i)), // todo use enum
                        // todo
                        // args: Box::new(args.clone()),
                        args: Box::new(()),
                        error: Box::new(e),
                    })
            })
    }
}

impl HtmlNode for NodeRef {
    fn selected<S: AsRef<str>>(&self, sel: S) -> Result<Vec<Self>, SelectError> {
        Ok(self
            .select(sel.as_ref())
            .map_err(|_| SelectError)?
            .into_iter()
            .map(|a| a.as_node().clone())
            .collect())
    }
}
