use crate::{ExtractionError, FromHtml, HtmlElementRef};
use scraper::ElementRef;

pub fn parse<H>(html: &str) -> Result<H, ExtractionError>
where
    for<'b, 'a> H: FromHtml<'b, (), Source<ElementRef<'a>> = ElementRef<'a>>,
{
    let doc = ::scraper::Html::parse_document(html);
    H::from_html::<ElementRef<'_>>(&doc.root_element(), ())
}
//
// pub trait Foo {
//     type Arg<N: Clone>;
//     fn foo<N: Clone>(arg: Self::Arg<N>) {
//         println!("hello");
//     }
// }

// pub fn call_foo<F: Foo<Arg<Box<usize>> = Box<usize>>>() {
//     // I want to create this Box value here
//     let a = Box::new(1);
//     F::foo::<Box<usize>>(a);
// }
