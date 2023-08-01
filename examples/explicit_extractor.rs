use h2s::extraction_method::ExtractNthText;
use h2s::FromHtml;

#[derive(FromHtml)]
struct Fragment {
    #[h2s(select = "div")]
    inner: Inner,
}

#[derive(FromHtml)]
struct Inner {
    #[h2s(extractor = ExtractNthText(0))]
    a: String,
    #[h2s(extractor = ExtractNthText(1))]
    b: String,
    #[h2s(extractor = ExtractNthText(2))]
    c: String,
}

fn main() {
    let my_struct = h2s::parse::<Fragment>(
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
    assert_eq!(&my_struct.inner.a, "A");
    assert_eq!(&my_struct.inner.b, "B");
    assert_eq!(&my_struct.inner.c, "C");
}

#[test]
fn test() {
    main();
}
