//! You can select backend HTML parser library to use, or you can also implement custom backend by yourself.
use h2s_core::html::HtmlDocument;

#[cfg(feature = "backend-scraper")]
pub mod scraper;

pub trait Backend {
    type Root: HtmlDocument;
    fn parse_document<S>(s: S) -> Self::Root
    where
        S: AsRef<str>;
}

// TODO create common test to check that backend satisfies required specs
