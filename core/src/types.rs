use crate::{ExtractFrom, H2s, H2sError, NotMatchedDetail};
use kuchiki::iter::{Descendants, Elements, Select};

impl ExtractFrom for String {
    fn extract_from(mut select: Select<Elements<Descendants>>) -> Result<Self, H2sError> {
        Ok(select
            .next()
            .ok_or_else(|| H2sError::NotMatched(NotMatchedDetail::MissingElement))?
            .as_node()
            .text_contents())
    }
}

impl<T: H2s> ExtractFrom for T {
    fn extract_from(mut select: Select<Elements<Descendants>>) -> Result<Self, H2sError> {
        T::parse(
            select
                .next()
                .ok_or_else(|| H2sError::NotMatched(NotMatchedDetail::MissingElement))?
                .as_node(),
        )
    }
}

impl ExtractFrom for Vec<String> {
    fn extract_from(mut select: Select<Elements<Descendants>>) -> Result<Self, H2sError> {
        Ok(select
            .map(|e| e.as_node().text_contents())
            .collect::<Vec<_>>())
    }
}

impl<T: H2s> ExtractFrom for Vec<T> {
    fn extract_from(select: Select<Elements<Descendants>>) -> Result<Self, H2sError> {
        select
            .map(|e| T::parse(e.as_node()))
            .fold(Ok(vec![]), |acc, parse_result| {
                acc.and_then(|v| parse_result.map(|t| (v, t)))
                    .map(|(mut v, t)| {
                        v.push(t);
                        v
                    })
            })
    }
}
