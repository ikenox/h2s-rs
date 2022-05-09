use crate::ExtractionError::HtmlStructureUnmatched;
use crate::{
    ExtractionError, FromHtml, GetElementError, HtmlElements, TendrilSink, TextExtractionMethod,
};
use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::{ElementData, NodeDataRef, NodeRef};

impl FromHtml for String {
    type Args = TextExtractionMethod;
    fn extract_from<N: HtmlElements>(
        mut select: N,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        let node = select
            .get_exactly_one()
            .map_err(|e| HtmlStructureUnmatched(e))?;
        Ok(match args {
            TextExtractionMethod::TextContent => node.as_node().text_contents(),
            TextExtractionMethod::Attribute(attr) => node
                .attributes
                .borrow()
                .get(attr.as_str())
                .ok_or_else(|| ExtractionError::AttributeNotFound)?
                .to_string(),
        })
    }
}

impl<T: FromHtml> FromHtml for Option<T> {
    type Args = T::Args;

    fn extract_from<N: HtmlElements>(
        select: N,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        let node = select
            .exactly_one_or_none()
            .map_err(|e| HtmlStructureUnmatched(e))?;
        Ok(if let Some(a) = node {
            Some(T::extract_from(a, &args)?)
        } else {
            None
        })
    }
}

impl<T: FromHtml> FromHtml for Vec<T> {
    type Args = T::Args;
    fn extract_from<N: HtmlElements>(
        select: N,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        select
            .list()
            .into_iter()
            .map(|e| T::extract_from(e, &args))
            .fold(Ok(vec![]), |acc, parse_result| {
                acc.and_then(|v| parse_result.map(|t| (v, t)))
                    .map(|(mut v, t)| {
                        v.push(t);
                        v
                    })
            })
    }
}

impl HtmlElements for NodeDataRef<ElementData> {
    fn exactly_one_or_none(self) -> Result<Option<NodeDataRef<ElementData>>, GetElementError> {
        Ok(Some(self))
    }
    fn get_exactly_one(self) -> Result<NodeDataRef<ElementData>, GetElementError> {
        Ok(self)
    }

    fn list(self) -> Vec<NodeDataRef<ElementData>> {
        vec![self]
    }
}

impl HtmlElements for Select<Elements<Descendants>> {
    fn exactly_one_or_none(mut self) -> Result<Option<NodeDataRef<ElementData>>, GetElementError> {
        Ok(self.next())
    }

    fn get_exactly_one(mut self) -> Result<NodeDataRef<ElementData>, GetElementError> {
        Ok(self.next().ok_or_else(|| GetElementError::NoElementFound)?)
    }

    fn list(self) -> Vec<NodeDataRef<ElementData>> {
        self.collect()
    }
}

impl HtmlElements for NodeRef {
    fn exactly_one_or_none(self) -> Result<Option<NodeDataRef<ElementData>>, GetElementError> {
        Ok(NodeDataRef::new_opt(self, |f| f.as_element()))
    }

    fn get_exactly_one(self) -> Result<NodeDataRef<ElementData>, GetElementError> {
        NodeDataRef::new_opt(self, |f| f.as_element())
            .ok_or_else(|| GetElementError::NoElementFound)
    }

    fn list(self) -> Vec<NodeDataRef<ElementData>> {
        NodeDataRef::new_opt(self, |f| f.as_element())
            .into_iter()
            .collect()
    }
}
