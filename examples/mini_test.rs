use h2s::FromHtml;

fn main() {
    #[derive(FromHtml, Debug)]
    pub struct MyStruct1 {
        #[h2s(select = "h1")]
        h1: usize,
    }

    let err = h2s::parse::<MyStruct1>("<div><h1>1</h1><h1>2</h1></div>").unwrap_err();
    println!("{}", err.to_string());
    // => (a) .a: structure unmatched: expected exactly one element, but 2 elements found

    #[derive(FromHtml, Debug)]
    pub struct MyStruct2 {
        #[h2s(select = "h2")]
        h2: [usize; 3],
    }

    let err = h2s::parse::<MyStruct2>("<div><h2>1</h2><h2>2</h2></div>").unwrap_err();
    println!("{}", err.to_string());
    // => (h2) h1: structure unmatched: expected 3 elements, but found 2 elements
}
