//! You can select backend HTML parser library to use, or you can also implement custom backend by yourself.
use h2s_core::HtmlNode;

#[cfg(feature = "backend-scraper")]
pub mod scraper;

pub trait Backend {
    type Root: DocumentRoot;
    fn parse_document<S: AsRef<str>>(s: S) -> Self::Root;
}

pub trait DocumentRoot {
    type HtmlNode<'a>: HtmlNode
    where
        Self: 'a;

    fn root_element(&self) -> Self::HtmlNode<'_>;
}

// TODO create common test to check that backend satisfies required specs
