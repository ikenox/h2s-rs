#![feature(generic_associated_types)]
pub mod utils;

use std::fmt::{Debug, Display, Formatter};

use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;

pub fn extract<S, T, B, N>(source: S, args: B) -> Result<T, ExtractionError>
where
    N: HtmlNodeRef,
    S: SourceAdjuster<T::Source<N>>,
    T: FromHtml,
    B: IntoArgs<T::Args>,
    T::Args: 'static,
{
    let adjusted = source
        .adjust_to()
        .map_err(|a| ExtractionError::GetElementError(a))?;
    let args = args.build_args();
    T::extract_from(&adjusted, &args)
        // .map_err(|_| todo!())
        .map_err(|inner| ExtractionError::Child {
            source: adjusted.info(),
            args: args.info(),
            error: Box::new(inner),
        })
}

pub trait ExtractionSource<N: HtmlNodeRef> {
    fn info(&self) -> SourceInfo;
}

pub trait FromHtml: Sized {
    // TODO future: change to associated type Source<T: HtmlNodeRef>
    type Source<N: HtmlNodeRef>: ExtractionSource<N>;
    type Args: ExtractionArgs;
    fn extract_from<N: HtmlNodeRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError>;
}

pub trait IntoArgs<A> {
    fn build_args(&self) -> A;
}

// todo not force to clone?
pub trait HtmlNodeRef: Sized + Clone {
    fn selected<S: AsRef<str>>(&self, sel: S) -> Result<Vec<Self>, SelectError>;
    fn text_contents(&self) -> String;
    fn get_attribute<S: AsRef<str>>(&self, sel: S) -> Option<String>;
}

pub trait ExtractFromNode<T>: Sized {
    fn extract(&self, n: &NodeRef) -> Result<Self, ExtractionError>;
}

pub trait ExtractionArgs {
    fn info(&self) -> ArgsInfo;
}

pub trait SourceAdjuster<T> {
    fn adjust_to(&self) -> Result<T, GetElementError>;
}

#[derive(Debug)]
pub enum ExtractionError {
    GetElementError(GetElementError),
    TextExtractionError(TextExtractionError),
    Child {
        source: SourceInfo,
        args: ArgsInfo,
        error: Box<ExtractionError>,
    },
}

#[derive(Debug)]
pub struct SourceInfo;

#[derive(Debug)]
pub struct ArgsInfo;

#[derive(Debug)]
pub enum GetElementError {
    NoElementFound,
    TooManyElements,
    Unexpected(String),
}

impl Display for GetElementError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            GetElementError::NoElementFound => write!(f, "no element found"),
            GetElementError::TooManyElements => write!(f, "too many elements"),
            GetElementError::Unexpected(s) => write!(f, "unexpected error: {s}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextExtractionError {
    AttributeNotFound(String),
}

impl Display for ExtractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo
        match self {
            Self::GetElementError(e) => {
                write!(f, "failed to get element: {e}")
            }
            Self::TextExtractionError(e) => {
                write!(f, "failed to extract text: {e}")
            }
            Self::Child {
                source,
                args,
                error,
            } => {
                // todo
                write!(f, "\n {source:?} {args:?} > {error}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectError;

#[derive(Debug, Clone)]
pub enum TextExtractionMethod {
    TextContent,
    Attribute(String),
}

impl Display for TextExtractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::AttributeNotFound(attr) => {
                write!(f, "attribute `{attr}` is not found in the target element")
            }
        }
    }
}

impl TextExtractionMethod {
    pub fn extract<T: HtmlNodeRef>(&self, node: &T) -> Result<String, TextExtractionError> {
        let s = match &self {
            TextExtractionMethod::TextContent => node.text_contents(),
            TextExtractionMethod::Attribute(attr) => node
                .get_attribute(attr)
                .ok_or_else(|| TextExtractionError::AttributeNotFound(attr.clone()))?,
        };
        Ok(s)
    }
}

#[derive(Debug, Clone)]
pub struct StructExtractionArgs;

impl ExtractionArgs for StructExtractionArgs {
    fn info(&self) -> ArgsInfo {
        ArgsInfo
    }
}
impl ExtractionArgs for TextExtractionMethod {
    fn info(&self) -> ArgsInfo {
        ArgsInfo
    }
}

impl HtmlNodeRef for NodeRef {
    fn selected<S: AsRef<str>>(&self, sel: S) -> Result<Vec<Self>, SelectError> {
        Ok(self
            .select(sel.as_ref())
            .map_err(|_| SelectError)?
            .into_iter()
            .map(|a| a.as_node().clone())
            .collect())
    }

    fn text_contents(&self) -> String {
        self.text_contents()
    }

    fn get_attribute<S: AsRef<str>>(&self, sel: S) -> Option<String> {
        self.as_element().and_then(|e| {
            e.attributes
                .borrow()
                .get(sel.as_ref())
                .map(|a| a.to_string())
        })
    }
}

impl<N: HtmlNodeRef> ExtractionSource<N> for N {
    fn info(&self) -> SourceInfo {
        SourceInfo
    }
}
impl<N: HtmlNodeRef, E: ExtractionSource<N>> ExtractionSource<N> for Option<E> {
    fn info(&self) -> SourceInfo {
        SourceInfo
    }
}
impl<N: HtmlNodeRef, E: ExtractionSource<N>> ExtractionSource<N> for Vec<E> {
    fn info(&self) -> SourceInfo {
        SourceInfo
    }
}

impl FromHtml for String {
    type Source<N: HtmlNodeRef> = N;
    type Args = TextExtractionMethod;

    fn extract_from<N: HtmlNodeRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        args.extract(source)
            .map_err(|e| ExtractionError::TextExtractionError(e))
    }
}

impl<H: FromHtml> FromHtml for Vec<H> {
    type Source<N: HtmlNodeRef> = Vec<H::Source<N>>;
    type Args = H::Args;

    fn extract_from<N: HtmlNodeRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        Ok(source
            .into_iter()
            .map(|a| H::extract_from(a, args).unwrap())
            .collect())
    }
}

impl<H: FromHtml> FromHtml for Option<H> {
    type Source<N: HtmlNodeRef> = Option<H::Source<N>>;
    type Args = H::Args;

    fn extract_from<N: HtmlNodeRef>(
        source: &Self::Source<N>,
        args: &Self::Args,
    ) -> Result<Self, ExtractionError> {
        Ok(source.as_ref().map(|a| H::extract_from(&a, args).unwrap()))
    }
}

#[derive(Debug)]
pub struct ArgBuilder {
    pub attr: Option<String>,
}

#[derive(Debug)]
pub struct SourceExtractor<N: HtmlNodeRef> {
    pub node: N,
    pub selector: Option<String>,
}

impl<N: HtmlNodeRef> SourceAdjuster<N> for SourceExtractor<N> {
    fn adjust_to(&self) -> Result<N, GetElementError> {
        if let Some(selector) = &self.selector {
            let mut elems = self
                .node
                .selected(selector)
                .map_err(|_| GetElementError::Unexpected(format!("select failed")))?;
            if elems.len() > 1 {
                Err(GetElementError::TooManyElements)
            } else {
                elems.pop().ok_or_else(|| GetElementError::NoElementFound)
            }
        } else {
            Ok(self.node.clone())
        }
    }
}

impl<N: HtmlNodeRef> SourceAdjuster<Option<N>> for SourceExtractor<N> {
    fn adjust_to(&self) -> Result<Option<N>, GetElementError> {
        if let Some(selector) = &self.selector {
            let mut elems = self
                .node
                .selected(selector)
                .map_err(|_| GetElementError::Unexpected(format!("select failed")))?;
            if elems.len() > 1 {
                Err(GetElementError::TooManyElements)
            } else {
                Ok(elems.pop())
            }
        } else {
            Ok(Some(self.node.clone()))
        }
    }
}

impl<N: HtmlNodeRef> SourceAdjuster<Vec<N>> for SourceExtractor<N> {
    fn adjust_to(&self) -> Result<Vec<N>, GetElementError> {
        if let Some(selector) = &self.selector {
            self.node
                .selected(selector)
                .map_err(|_| GetElementError::Unexpected(format!("select failed")))
        } else {
            // todo undefined behavior?
            Ok(vec![self.node.clone()])
        }
    }
}

impl<'a> IntoArgs<StructExtractionArgs> for ArgBuilder {
    fn build_args(&self) -> StructExtractionArgs {
        StructExtractionArgs
    }
}

impl<'a> IntoArgs<TextExtractionMethod> for ArgBuilder {
    fn build_args(&self) -> TextExtractionMethod {
        if let Some(attr) = &self.attr {
            TextExtractionMethod::Attribute(attr.to_string())
        } else {
            TextExtractionMethod::TextContent
        }
    }
}
