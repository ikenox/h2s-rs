#![feature(generic_associated_types)]

use h2s::FromHtml;

#[test]
fn string_values() {
    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Struct1 {
        // Single String, attr
        #[h2s(attr = "lang")]
        a: String,
        // Single String, select
        #[h2s(select = ".b")]
        b: String,
        // Single String, attr & select
        #[h2s(select = ".c", attr = "id")]
        c: String,
        // Vec
        #[h2s(select = ".d")]
        d: Vec<String>,
        // Option - Some
        #[h2s(select = ".e")]
        e: Option<String>,
        // Option - None
        #[h2s(select = ".class-that-does-not-exist")]
        e_none: Option<String>,
        // Array
        #[h2s(select = ".f")]
        f: [String; 3],
        #[h2s(select = ".g")]
        g: Struct2,
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Struct2 {
        // An inner text of struct root element
        #[h2s()]
        h: String,
    }

    let html = r#"
<!DOCTYPE html>
<html lang="avalue">
<body>

<div class="b">bvalue</div>

<div class="c" id="cvalue" />

<div class="d">dvalue1</div>
<div class="d">dvalue2</div>
<div class="d">dvalue3</div>

<div class="e">evalue</div>

<div class="f">fvalue1</div>
<div class="f">fvalue2</div>
<div class="f">fvalue3</div>
    
<div class="g">hvalue</div>

</body>
</html>
    "#;

    let res = h2s::utils::parse::<Struct1>(html);
    assert_eq!(
        res,
        Ok(Struct1 {
            a: s("avalue"),
            b: s("bvalue"),
            c: s("cvalue"),
            d: vec![s("dvalue1"), s("dvalue2"), s("dvalue3")],
            e: Some(s("evalue")),
            e_none: None,
            f: [s("fvalue1"), s("fvalue2"), s("fvalue3")],
            g: Struct2 { h: s("hvalue") }
        })
    )
}

#[test]
fn nested_struct() {
    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Struct {
        // Single Struct, parse root element itself
        #[h2s()]
        a: StructInner1,
        // Single Struct, select
        #[h2s(select = ".b")]
        b: StructInner2,
        // Vec
        #[h2s(select = ".c")]
        c: Vec<StructInner2>,
        // Option - Some
        #[h2s(select = ".d")]
        d: Option<StructInner2>,
        // Option - None
        #[h2s(select = ".class-that-does-not-exist")]
        d_none: Option<StructInner2>,
        // Array
        #[h2s(select = ".e")]
        e: [StructInner2; 3],
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct StructInner1 {
        #[h2s(attr = "lang")]
        a: String,
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct StructInner2 {
        #[h2s(select = "span")]
        v: String,
    }

    let html = r#"
<!DOCTYPE html>
<html lang="a">
<body>
<div class="b"><span>bvalue</span></div>

<div class="c"><span>cvalue1</span></div>
<div class="c"><span>cvalue2</span></div>
<div class="c"><span>cvalue3</span></div>

<div class="d"><span>dvalue</span></div>

<div class="e"><span>evalue1</span></div>
<div class="e"><span>evalue2</span></div>
<div class="e"><span>evalue3</span></div>
</body>
</html>
    "#;

    let res = h2s::utils::parse::<Struct>(html);

    let si2 = |s: &str| StructInner2 { v: s.to_string() };

    assert_eq!(
        res,
        Ok(Struct {
            a: StructInner1 { a: s("a") },
            b: si2("bvalue"),
            c: vec![si2("cvalue1"), si2("cvalue2"), si2("cvalue3")],
            d: Some(si2("dvalue")),
            d_none: None,
            e: [si2("evalue1"), si2("evalue2"), si2("evalue3")],
        })
    )
}

#[test]
#[ignore]
fn invalid_macro_attribute_pattern() {
    // TODO
}

#[test]
#[ignore]
fn macro_error_message() {
    // TODO
}

#[test]
#[ignore]
fn parse_error_message() {
    // TODO
}

fn s(s: &str) -> String {
    s.to_string()
}
