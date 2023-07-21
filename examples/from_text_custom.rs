use std::num::ParseIntError;

use h2s::FromHtml;
use h2s_core::html::HtmlElement;
use h2s_core::parseable::Parseable;

fn main() {
    // You can define an external parseable type yourself
    // Currently you have to define a newtype for an external crate struct
    struct Duration(std::time::Duration);
    impl Parseable for Duration {
        type Error = ParseIntError;

        type Input<N: HtmlElement> = String;

        fn parse<N: HtmlElement>(input: Self::Input<N>) -> Result<Self, Self::Error> {
            let sec = input.parse()?;
            Ok(Duration(std::time::Duration::from_secs(sec)))
        }
    }

    #[derive(FromHtml)]
    struct MyStruct {
        #[h2s(select = "div")]
        duration1: Duration,
        #[h2s(select = "div", attr = "seconds")]
        duration2: Duration,
    }

    let my_struct = h2s::parse::<MyStruct>(r#"<div seconds="456">123</div>"#).unwrap();
    assert_eq!(my_struct.duration1.0, std::time::Duration::from_secs(123));
    assert_eq!(my_struct.duration2.0, std::time::Duration::from_secs(456));
}

#[test]
fn test() {
    main();
}
