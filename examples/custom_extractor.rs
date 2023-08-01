use h2s::extraction_method::ExtractionMethod;
use h2s::html::{HtmlElement, HtmlNode, TextNode};
use h2s::FromHtml;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(FromHtml)]
struct TreeNode {
    #[h2s(select="div", extractor = ExtractNthText(0))]
    a: String,
    #[h2s(select="div", extractor = ExtractNthText(1))]
    b: String,
    #[h2s(select="div", extractor = ExtractNthText(2))]
    c: String,

    #[h2s(select = "ul > li")]
    xy: Vec<String>,
}

/// Extracts nth text node's text
#[derive(Debug)]
struct ExtractNthText(usize);

impl ExtractionMethod for ExtractNthText {
    type Error = NotFound;
    type ExtractedValue<N: HtmlElement> = String;

    fn extract<N: HtmlElement>(&self, element: N) -> Result<Self::ExtractedValue<N>, Self::Error> {
        dbg!(element.child_nodes());
        element
            .child_nodes()
            .iter()
            .filter_map(|n| match n {
                HtmlNode::Text(text) => Some(text),
                _ => None,
            })
            .map(|t| t.get_text())
            .filter(|s| !s.trim().is_empty())
            .nth(self.0)
            .map(|s| s.trim().to_string())
            .ok_or(NotFound)
    }
}

impl Display for ExtractNthText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExtractNthText({})", self.0)
    }
}

#[derive(Debug)]
pub struct NotFound;

impl Display for NotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotFound")
    }
}

impl Error for NotFound {}

fn main() {
    let my_struct = h2s::parse::<TreeNode>(
        r#"<div>
             A
             <ul>
               <li>X</li>
               <li>Y</li>
             </ul>
             B
             <p>Z</p>
             C
           </div>
          "#,
    )
    .unwrap();
    assert_eq!(&my_struct.a, "A");
    assert_eq!(&my_struct.b, "B");
    assert_eq!(&my_struct.c, "C");
    assert_eq!(my_struct.xy, vec!["X".to_string(), "Y".to_string()]);
}

#[test]
fn test() {
    main();
}
