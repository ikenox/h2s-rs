use crate::html::HtmlElement;
use crate::parseable::ExtractedValue;
use crate::{Error, Never};
use std::fmt::{Debug, Display};

pub trait ExtractionMethod: Debug + Display {
    type Error: Error;
    type ExtractedValue<N: HtmlElement>: ExtractedValue;

    fn extract<N: HtmlElement>(&self, element: N) -> Result<Self::ExtractedValue<N>, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct NoOp;

impl ExtractionMethod for NoOp {
    type Error = Never;
    type ExtractedValue<N: HtmlElement> = N;

    fn extract<N>(&self, element: N) -> Result<Self::ExtractedValue<N>, Self::Error>
    where
        N: HtmlElement,
    {
        Ok(element)
    }
}

#[derive(Debug, Clone)]
pub struct ExtractInnerText;

impl ExtractionMethod for ExtractInnerText {
    type Error = Never;
    type ExtractedValue<N: HtmlElement> = String;

    fn extract<N>(&self, element: N) -> Result<Self::ExtractedValue<N>, Self::Error>
    where
        N: HtmlElement,
    {
        Ok(element.text_contents())
    }
}

#[derive(Debug, Clone)]
pub struct ExtractAttribute {
    pub name: String,
}

impl ExtractionMethod for ExtractAttribute {
    type Error = AttributeNotFound;
    type ExtractedValue<N: HtmlElement> = String;

    fn extract<N: HtmlElement>(&self, element: N) -> Result<Self::ExtractedValue<N>, Self::Error>
    where
        N: HtmlElement,
    {
        element
            .attribute(&self.name)
            .map(|a| a.to_string())
            .ok_or_else(|| AttributeNotFound {
                name: self.name.clone(),
            })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttributeNotFound {
    pub name: String,
}
