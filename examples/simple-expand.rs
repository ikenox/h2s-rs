#![feature(prelude_import)]
#![feature(generic_associated_types)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use h2s::FromHtml;
fn main() {
    pub struct Page {
        lang: String,
        blog_title: String,
        article_authors: Vec<String>,
        // articles: Vec<ArticleElem>,
    }
    impl ::h2s::FromHtml for Page {
        fn from_html<N: ::h2s::HtmlNodeRef>(input: &N) -> Result<Self, ::h2s::ExtractionError> {
            Ok(Self {
                lang: ::h2s::extract::<N, _, _>(
                    ::h2s::select(input, &"html".to_string())?,
                    ::h2s::TextContentExtractor,
                )?,
                blog_title: ::h2s::extract::<N, _, _>(
                    ::h2s::select(input, &"h1.blog-title".to_string())?,
                    ::h2s::TextContentExtractor,
                )?,
                article_authors: ::h2s::extract::<Vec<N>, _, _>(
                    ::h2s::select(input, &".articles > div".to_string())?,
                    ::h2s::AttributeExtractor {
                        attr: "data-author".to_string(),
                    },
                )?,
                // articles: ::h2s::extract::<_, Vec<N>, _, _>(
                //     ::h2s::select(input, &".articles > div".to_string())?,
                //     ::h2s::StructExtractor::new(),
                // )?,
            })
        }
    }
}
