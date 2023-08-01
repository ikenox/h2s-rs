use h2s::extraction_method::ExtractNthText;
use h2s::FromHtml;

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
