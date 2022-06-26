#![feature(generic_associated_types)]

use h2s_core::{FromHtml, HtmlNode, ParseError, Selector};

fn main() {
    // You can also implement a parser yourself
    struct MyStruct {
        foo: String,
        bar: String,
    }

    impl FromHtml<()> for MyStruct {
        type Source<N: HtmlNode> = N;

        fn from_html<N: HtmlNode>(
            source: &Self::Source<N>,
            _args: &(),
        ) -> Result<Self, ParseError> {
            Ok(MyStruct {
                foo: source.text_contents(),
                bar: source
                    .select(&N::Selector::parse("div").unwrap()) // TODO remove unwrap
                    .get(0)
                    .and_then(|e| e.get_attribute("bar"))
                    .ok_or_else(|| ParseError::Root {
                        message: "no attribute".to_string(),
                        cause: None,
                    })?
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
