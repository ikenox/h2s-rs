#![feature(generic_associated_types)]

use h2s_core::{FromHtml, FromHtmlError, HtmlNode, Selector};
use std::fmt::{Display, Formatter};

fn main() {
    // You can also implement a parser yourself
    struct MyStruct {
        foo: String,
        bar: String,
    }

    #[derive(Debug)]
    struct MyStructError(String);

    impl FromHtmlError for MyStructError {}

    impl Display for MyStructError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromHtml<()> for MyStruct {
        type Source<N: HtmlNode> = N;
        type Error = MyStructError;

        fn from_html<N: HtmlNode>(
            source: &Self::Source<N>,
            _args: &(),
        ) -> Result<Self, Self::Error> {
            Ok(MyStruct {
                foo: source.text_contents(),
                bar: source
                    .select(&N::Selector::parse("div").unwrap()) // TODO remove unwrap
                    .get(0)
                    .and_then(|e| e.get_attribute("bar"))
                    .ok_or_else(|| MyStructError("no attribute".to_string()))?
                    .to_string(),
            })
        }
    }

    let my_struct = h2s::util::parse::<MyStruct>(r#"<div bar="world">hello<div>"#).unwrap();
    assert_eq!(&my_struct.foo, "hello");
    assert_eq!(&my_struct.bar, "world");
}

#[test]
fn test() {
    main();
}
