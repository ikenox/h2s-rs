#![feature(generic_associated_types)]

#[test]
fn derive() {
    use h2s::FromHtml;

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Struct1 {
        // attr
        #[h2s(attr = "lang")]
        a: String,
        // select
        #[h2s(select = ".b")]
        b: String,
        // attr & select
        #[h2s(select = ".c", attr = "id")]
        c: String,

        // Vec
        #[h2s(select = ".d > div")]
        d: Vec<String>,
        // Option
        #[h2s(select = ".e")]
        e: Option<String>,
        // Array
        #[h2s(select = ".f > div")]
        f: [String; 3],
        // Another derived struct
        #[h2s(select = ".g")]
        g: Struct2,
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Struct2 {
        #[h2s()]
        h: String,
    }

    let html = r#"
<!DOCTYPE html>
<html lang="avalue">
<body>
<div class="b">bvalue</div>
<div class="c" id="cvalue" />
<div class="d">
    <div>dvalue1</div>
    <div>dvalue2</div>
    <div>dvalue3</div>
</div>
<div class="e">evalue</div>
<div class="f">
    <div>fvalue1</div>
    <div>fvalue2</div>
    <div>fvalue3</div>
</div>
<div class="g">hvalue</div>
</body>
</html>
    "#;

    let res = h2s::utils::parse::<Struct1>(html);
    assert_eq!(
        res,
        Ok(Struct1 {
            a: "avalue".to_string(),
            b: "bvalue".to_string(),
            c: "cvalue".to_string(),
            d: vec(["dvalue1", "dvalue2", "dvalue3"]),
            e: Some("evalue".to_string()),
            f: ["fvalue1", "fvalue2", "fvalue3"].map(|s| s.to_string()),
            g: Struct2 {
                h: "hvalue".to_string()
            }
        })
    )
}

fn vec<const N: usize>(arr: [&str; N]) -> Vec<String> {
    arr.map(|s| s.to_string()).to_vec()
}
